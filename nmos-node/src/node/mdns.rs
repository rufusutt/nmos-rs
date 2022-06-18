use std::{sync::Arc, any::Any, time::Duration};

use tokio::sync::mpsc::{channel, Sender, Receiver};

use tracing::info;
use zeroconf::{
    service::TMdnsService, txt_record::TTxtRecord, MdnsService, ServiceDiscovery, ServiceType,
    TxtRecord, MdnsBrowser, browser::TMdnsBrowser, EventLoop, event_loop::TEventLoop,
};

pub struct MdnsContext<'a> {
    // Browsers and services
    register_browser: MdnsBrowser,
    node_service: MdnsService,
    // Event loops
    event_loops: Vec<EventLoop<'a>>,
}

pub struct MdnsReceivers {
    pub register_rx: Receiver<ServiceDiscovery>,
}

impl MdnsContext<'_> {
    fn on_service_discovered(
        result: zeroconf::Result<ServiceDiscovery>,
        context: Option<Arc<dyn Any>>,
    ) {
        // Get discovery
        let sd = match result {
            Ok(sd) => sd,
            Err(_) => return,
        };

        info!("Registry discovered: {:?}", sd);

        // Cast context
        let tx = context
            .as_ref()
            .expect("Expected MdnsContext")
            .downcast_ref::<Sender<ServiceDiscovery>>()
            .unwrap()
            .clone();
            
        tx.send(sd);
    }

    pub fn new<'a>() -> (MdnsContext<'a>, MdnsReceivers) {
        let mut event_loops: Vec<EventLoop> = Vec::new();

        let mut register_browser = MdnsBrowser::new(ServiceType::new("nmos-register", "tcp").unwrap());

        let (tx, register_rx): (Sender<ServiceDiscovery>, Receiver<ServiceDiscovery>) = channel(100);
        register_browser.set_service_discovered_callback(Box::new(Self::on_service_discovered));
        register_browser.set_context(Box::new(tx));

        event_loops.push(register_browser.browse_services().expect("Register event handler"));


        let mut node_service = MdnsService::new(ServiceType::new("nmos-node", "tcp").unwrap(), 3000);
        let txt_record = TxtRecord::new();

        node_service.set_txt_record(txt_record);

        let context = MdnsContext {
            register_browser,
            node_service,
            event_loops,
        };

        let receivers = MdnsReceivers {
            register_rx,
        };

        (context, receivers)
    }

    pub fn poll(&mut self) {
        for event_loop in self.event_loops {
            event_loop.poll(Duration::from_secs(0)).unwrap();
        }
    }
}
