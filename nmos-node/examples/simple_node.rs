use nmos_node::async_trait;
use nmos_node::node::{EventHandler, Node};
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

    let mut node = Node::builder().event_handler(Handler).build();

    if let Err(e) = node.start().await {
        println!("Node error: {:?}", e);
    }
}
