use std::{any::Any, sync::Arc, time::Duration};

use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::info;
use zeroconf::{
    browser::TMdnsBrowser, event_loop::TEventLoop, service::TMdnsService, txt_record::TTxtRecord,
    EventLoop, MdnsBrowser, MdnsService, ServiceDiscovery, ServiceRegistration, ServiceType,
    TxtRecord,
};

pub struct MdnsConfig {}

#[derive(Debug)]
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
        // Cast context
        let tx = context
            .as_ref()
            .expect("Missing context")
            .downcast_ref::<UnboundedSender<NmosMdnsEvent>>()
            .unwrap();

        tx.send(NmosMdnsEvent::Registration(service, result))
            .expect("Unable to send MDNS event");
    }

    pub fn new(config: &MdnsConfig, tx: mpsc::UnboundedSender<NmosMdnsEvent>) -> MdnsContext {
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
            node_service: None,
            query_service: None,
        }
    }

    pub fn start<'a>(&'a mut self) -> MdnsPoller {
        let mut event_loops = Vec::new();

        if let Some(register_browser) = &mut self.register_browser {
            event_loops.push(
                register_browser
                    .browse_services()
                    .expect("Register event handler"),
            );
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
