use async_trait::async_trait;

use crate::dto::organization::{PatchBranchDTO, PatchOrganizationDTO, PostBranchDTO, PostOrganizationDTO};
use crate::models::organization::{Branch, Organization};
use crate::repositories::organization::{BranchQueryParams, OrganizationQueryParams};
use crate::repositories::repository::ResultPaging;
use common::errors::BasicError;
use common::responses::DeleteResponseResult;

// ==================== ORGANIZATION ==================== //

#[async_trait]
pub trait OrganizationService: Sync + Send {
    async fn create(&self, data: PostOrganizationDTO, user_id: String) -> Result<Organization, BasicError>;
    async fn delete(&self, current_id: String, user_id: String) -> Result<DeleteResponseResult, BasicError>;
    async fn get(&self, current_id: String, user_id: String) -> Result<Organization, BasicError>;
    async fn list(
        &self,
        query_params: OrganizationQueryParams,
        user_id: String,
    ) -> Result<ResultPaging<Organization>, BasicError>;
    async fn patch(&self, id: String, data: PatchOrganizationDTO, user_id: String) -> Result<Organization, BasicError>;
}

// ==================== BRANCH ==================== //

#[async_trait]
pub trait BranchService: Sync + Send {
    async fn create(&self, data: PostBranchDTO, user_id: String, organization_id: String)
        -> Result<Branch, BasicError>;
    async fn delete(
        &self,
        current_id: String,
        user_id: String,
        organization_id: String,
    ) -> Result<DeleteResponseResult, BasicError>;
    async fn get(&self, current_id: String, user_id: String) -> Result<Branch, BasicError>;
    async fn list(&self, query_params: BranchQueryParams, user_id: String) -> Result<ResultPaging<Branch>, BasicError>;
    async fn patch(
        &self,
        current_id: String,
        data: PatchBranchDTO,
        user_id: String,
        organization_id: String,
    ) -> Result<Branch, BasicError>;
}
