use std::{any::Any, sync::Arc, time::Duration};

use futures::{channel::oneshot::{channel, Receiver, Sender}, executor::block_on};

use tracing::info;
use zeroconf::{
    browser::TMdnsBrowser, event_loop::TEventLoop, service::TMdnsService, txt_record::TTxtRecord,
    EventLoop, MdnsBrowser, MdnsService, ServiceDiscovery, ServiceType, TxtRecord,
};

#[derive(Debug)]
pub struct MdnsContext {
    // Browsers and services
    register_browser: Option<MdnsBrowser>,
    node_service: Option<MdnsService>,
    // Receivers
    receivers: Arc<MdnsReceivers>,
}

#[derive(Debug)]
pub struct MdnsReceivers {
    pub register_rx: Receiver<ServiceDiscovery>,
}

pub struct MdnsPoller<'a> {
    event_loops: Vec<EventLoop<'a>>,
}

impl MdnsContext {
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

        // block_on(tx.send(sd));
    }

    pub fn new() -> MdnsContext {
        let mut register_browser =
            MdnsBrowser::new(ServiceType::new("nmos-register", "tcp").unwrap());

        let (tx, register_rx): (Sender<ServiceDiscovery>, Receiver<ServiceDiscovery>) = channel();
        register_browser.set_service_discovered_callback(Box::new(Self::on_service_discovered));
        register_browser.set_context(Box::new(tx));

        let mut node_service =
            MdnsService::new(ServiceType::new("nmos-node", "tcp").unwrap(), 3000);
        let txt_record = TxtRecord::new();

        node_service.set_txt_record(txt_record);

        let receivers = MdnsReceivers { register_rx };

        MdnsContext {
            register_browser: Some(register_browser),
            node_service: None,
            receivers: Arc::new(receivers),
        }
    }

    pub fn receivers(&self) -> Arc<MdnsReceivers> {
        return self.receivers.clone();
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
