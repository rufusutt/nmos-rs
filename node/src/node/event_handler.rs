use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {

}
