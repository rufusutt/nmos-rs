use nmos_model::resource::{
    DeviceBuilder, Format, NodeBuilder, ReceiverBuilder, ResourceBundle, Transport,
};
use nmos_node::Node;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Set default subscriber");

    // Create our resources
    let node = NodeBuilder::new("Simple test node", "http://127.0.0.1:3000/").build();
    let device = DeviceBuilder::new("Simple test device", &node, "urn:x-nmos:device:generic").build();
    let receiver = ReceiverBuilder::new(
        "Simple test receiver",
        &device,
        Format::Video,
        Transport::Rtp,
    )
    .build();

    // Place inside bundle
    let mut resources = ResourceBundle::new();
    resources.insert_node(node);
    resources.insert_device(device);
    resources.insert_receiver(receiver);

    // Create node
    let node = Node::builder_from_resources(resources).build();

    if let Err(e) = node.start().await {
        println!("Node error: {:?}", e);
    }
}
