use async_trait::async_trait;

#[async_trait]
pub trait CommonService: Sync + Send {}
