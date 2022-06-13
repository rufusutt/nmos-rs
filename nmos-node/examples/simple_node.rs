use nmos_node::async_trait;
use nmos_node::node::{EventHandler, Node};

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let mut node = Node::builder()
        .event_handler(Handler)
        .await
        .expect("Error creating node");

    if let Err(e) = node.start().await {
        println!("Node error: {:?}", e);
    }
}
