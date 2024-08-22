use std::sync::Arc;

use async_trait::async_trait;
use domain::repositories::common::CommonRepository;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

// ==================== USER ==================== //
pub struct CommonSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl CommonSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        CommonSurrealRepository { pool: db }
    }
}

#[async_trait]
impl CommonRepository for CommonSurrealRepository {}
