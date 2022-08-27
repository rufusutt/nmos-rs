use std::{
    any::Any,
    cmp::Ordering,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use http::Uri;
use nmos_rs_model::version::APIVersion;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{error, info};
use zeroconf::{
    browser::TMdnsBrowser, event_loop::TEventLoop, service::TMdnsService, txt_record::TTxtRecord,
    EventLoop, MdnsBrowser, MdnsService, ServiceDiscovery, ServiceRegistration, ServiceType,
    TxtRecord,
};

pub struct NmosMdnsConfig {}

#[derive(Debug, Eq, PartialEq)]
pub struct NmosMdnsRegistry {
    api_proto: String,
    api_ver: Vec<APIVersion>,
    api_auth: bool,
    pri: u8,
    uri: Uri,
}

impl NmosMdnsRegistry {
    pub fn parse(discovery: &ServiceDiscovery) -> Option<Self> {
        // TXT record required
        let txt = match discovery.txt() {
            Some(txt) => txt,
            None => return None,
        };

        // Get required fields
        if let (Some(api_proto), Some(api_ver), Some(api_auth), Some(pri)) = (
            txt.get("api_proto"),
            txt.get("api_ver"),
            txt.get("api_auth"),
            txt.get("pri"),
        ) {
            // TODO: Validate fields

            let address = discovery.address();
            let port = discovery.port();

            // Use std to form valid address port combination
            let address = match IpAddr::from_str(address) {
                Ok(addr) => addr,
                Err(_) => return None,
            };
            let socket = SocketAddr::new(address, *port);
            let authority = socket.to_string();

            // Build URI
            let uri = match Uri::builder()
                .scheme(api_proto.as_str())
                .authority(authority)
                .path_and_query("/x-nmos/registration/")
                .build()
            {
                Ok(uri) => uri,
                Err(err) => {
                    error!("Cannot build URI: {}", err);
                    return None;
                }
            };

            // Parse api_ver
            let api_ver: Vec<APIVersion> =
                api_ver.split(',').flat_map(APIVersion::from_str).collect();

            // Parse api_auth
            let api_auth = match api_auth.parse::<bool>() {
                Ok(auth) => auth,
                Err(_) => return None,
            };

            // Parse pri
            let pri = match pri.parse::<u8>() {
                Ok(pri) => pri,
                Err(_) => return None,
            };

            Some(Self {
                api_proto,
                api_ver,
                api_auth,
                pri,
                uri,
            })
        } else {
            None
        }
    }
}

impl Ord for NmosMdnsRegistry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order entries by smallest priority
        other.pri.cmp(&self.pri)
    }
}

impl PartialOrd for NmosMdnsRegistry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MdnsContext {
    // Browsers and services
    register_browser: Option<MdnsBrowser>,
    node_service: Option<MdnsService>,
    query_service: Option<MdnsService>,
}

#[derive(Debug)]
pub enum NmosMdnsService {
    Node,
    Registration,
    Query,
}

#[derive(Debug)]
pub enum NmosMdnsEvent {
    Discovery(NmosMdnsService, zeroconf::Result<ServiceDiscovery>),
    Registration(NmosMdnsService, zeroconf::Result<ServiceRegistration>),
}

pub struct MdnsPoller<'a> {
    event_loops: Vec<EventLoop<'a>>,
}

impl MdnsContext {
    fn on_service_discovered(
        service: NmosMdnsService,
        result: zeroconf::Result<ServiceDiscovery>,
        context: Option<Arc<dyn Any>>,
    ) {
        match &result {
            Ok(d) => info!("Discovered service: {:?}", d),
            Err(e) => error!("Service discovery error: {}", e),
        };

        // Cast context
        let tx = context
            .as_ref()
            .expect("Missing context")
            .downcast_ref::<UnboundedSender<NmosMdnsEvent>>()
            .unwrap();

        tx.send(NmosMdnsEvent::Discovery(service, result))
            .expect("Unable to send MDNS event");
    }

    fn register_callback(
        service: NmosMdnsService,
        result: zeroconf::Result<ServiceRegistration>,
        context: Option<Arc<dyn Any>>,
    ) {
        match &result {
            Ok(r) => info!("{} service registered", r.service_type().to_string()),
            Err(e) => error!("Registration error: {}", e),
        }

        // Cast context
        let tx = context
            .as_ref()
            .expect("Missing context")
            .downcast_ref::<UnboundedSender<NmosMdnsEvent>>()
            .unwrap();

        tx.send(NmosMdnsEvent::Registration(service, result))
            .expect("Unable to send MDNS event");
    }

    pub fn new(config: &NmosMdnsConfig, tx: mpsc::UnboundedSender<NmosMdnsEvent>) -> MdnsContext {
        // Create registration browser
        let mut register_browser =
            MdnsBrowser::new(ServiceType::new("nmos-register", "tcp").unwrap());

        register_browser.set_context(Box::new(tx.clone()));
        register_browser.set_service_discovered_callback(Box::new(|r, c| {
            Self::on_service_discovered(NmosMdnsService::Registration, r, c)
        }));

        // Create node service
        let mut node_service =
            MdnsService::new(ServiceType::new("nmos-node", "tcp").unwrap(), 3000);
        let txt_record = TxtRecord::new();

        node_service.set_txt_record(txt_record);
        node_service.set_context(Box::new(tx));
        node_service.set_registered_callback(Box::new(|r, c| {
            Self::register_callback(NmosMdnsService::Node, r, c)
        }));

        MdnsContext {
            register_browser: Some(register_browser),
            node_service: Some(node_service),
            query_service: None,
        }
    }

    pub fn start(&mut self) -> MdnsPoller {
        let mut event_loops = Vec::new();

        if let Some(register_browser) = &mut self.register_browser {
            event_loops.push(
                register_browser
                    .browse_services()
                    .expect("Register event handler"),
            );
        }

        if let Some(node_service) = &mut self.node_service {
            event_loops.push(node_service.register().unwrap());
        }

        MdnsPoller { event_loops }
    }
}

impl MdnsPoller<'_> {
    pub fn poll(&self) {
        for event_loop in &self.event_loops {
            event_loop.poll(Duration::from_secs(0)).unwrap();
        }
    }
}
