use async_trait::async_trait;

// ==================== COMMON ==================== //
#[async_trait]
pub trait CommonRepository: Send + Sync {}
