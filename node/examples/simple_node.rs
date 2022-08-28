use nmos_rs_model::resource::{
    DeviceBuilder, Format, NodeBuilder, ReceiverBuilder, ResourceBundle, Transport,
};
use nmos_rs_node::async_trait;
use nmos_rs_node::node::{EventHandler, Node};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Set default subscriber");

    // Create our resources
    let node = NodeBuilder::new("Test Node").build();
    let device = DeviceBuilder::new(&node, "test type").build();
    let receiver = ReceiverBuilder::new(&device, Format::Video, Transport::Rtp).build();

    // Place inside bundle
    let mut resources = ResourceBundle::new();
    resources.insert_node(node);
    resources.insert_device(device);
    resources.insert_receiver(receiver);

    // Create node
    let node = Node::builder_from_resources(resources)
        .event_handler(Handler)
        .build();

    if let Err(e) = node.start().await {
        println!("Node error: {:?}", e);
    }
}
