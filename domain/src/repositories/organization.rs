use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::models::organization::{
    Branch, CreateBranch, CreateOrganization, Organization, PatchBranch, PatchOrganization,
};
use crate::repositories::repository::ResultPaging;

use common::responses::DeleteResponseResult;
use common::types::BasicResult;

// ==================== ORGANIZATION ==================== //
#[derive(Default, Debug, Serialize, Deserialize, IntoParams)]
pub struct OrganizationQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start: Option<u32>,
    pub bot_token: Option<String>,

    pub created_at_from: Option<String>,
    pub created_at_to: Option<String>,

    pub updated_at_from: Option<String>,
    pub updated_at_to: Option<String>,

    pub result: Option<String>,
}

#[async_trait]
pub trait OrganizationTrait: Send + Sync {
    async fn create(&self, organization: CreateOrganization) -> BasicResult<Organization>;
    async fn delete(&self, current_id: String) -> BasicResult<DeleteResponseResult>;
    async fn get(&self, current_id: String) -> BasicResult<Organization>;
    async fn list(&self, query_params: OrganizationQueryParams) -> BasicResult<ResultPaging<Organization>>;
    async fn patch(&self, current_id: String, data: PatchOrganization) -> BasicResult<Organization>;
}

// ==================== BRANCH ==================== //
#[derive(Default, Debug, Serialize, Deserialize, IntoParams)]
pub struct BranchQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start: Option<u32>,

    pub organization_id: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub result: Option<String>,
}

#[async_trait]
pub trait BranchTrait: Send + Sync {
    async fn create(&self, branch: CreateBranch) -> BasicResult<Branch>;
    async fn delete(&self, current_id: String) -> BasicResult<DeleteResponseResult>;
    async fn get(&self, current_id: String) -> BasicResult<Branch>;
    async fn list(&self, query_params: BranchQueryParams) -> BasicResult<ResultPaging<Branch>>;
    async fn patch(&self, current_id: String, branch: PatchBranch) -> BasicResult<Branch>;
}
