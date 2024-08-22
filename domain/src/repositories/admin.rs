use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::models::admin::Admin;
use crate::models::admin::{CreateAdmin, PatchAdmin, PutAdminPassword};
use crate::repositories::repository::ResultPaging;

use common::enums::AdminRoleType;
use common::responses::DeleteResponseResult;
use common::types::BasicResult;

// ==================== ADMIN ==================== //
#[derive(Default, Debug, Serialize, Deserialize, IntoParams)]
pub struct AdminQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start: Option<u32>,

    pub role: Option<AdminRoleType>,

    pub created_at_from: Option<String>,
    pub created_at_to: Option<String>,

    pub updated_at_from: Option<String>,
    pub updated_at_to: Option<String>,

    pub result: Option<String>,
}

#[async_trait]
pub trait AdminTrait: Send + Sync {
    async fn change_password(&self, id: String, data: PutAdminPassword) -> BasicResult<String>;
    async fn get_by_phone_number(&self, phone_number: &str) -> BasicResult<Admin>;
    async fn is_superadmin_in_db(&self) -> BasicResult<bool>;
    async fn merge_cli(&self, current_phone_number: String, admin: PatchAdmin) -> BasicResult<Admin>;

    // BASIC
    async fn create(&self, data: CreateAdmin) -> BasicResult<Admin>;
    async fn delete(&self, id: String) -> BasicResult<DeleteResponseResult>;
    async fn get(&self, id: &str) -> BasicResult<Admin>;
    async fn list(&self, query_params: AdminQueryParams) -> BasicResult<ResultPaging<Admin>>;
    async fn patch(&self, id: String, data: PatchAdmin) -> BasicResult<Admin>;
}
