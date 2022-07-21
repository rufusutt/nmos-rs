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

    let node = Node::builder().event_handler(Handler).build().await;

    if let Err(e) = node.start().await {
        println!("Node error: {:?}", e);
    }
}
