use std::sync::Arc;

use async_trait::async_trait;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::dao::organization::{BranchQueryParamsTrait, OrganizationQueryParamsTrait};
use crate::data::organization::{
    CreateBranchSurreal, CreateOrganizationSurreal, GetBranchSurreal, GetOrganizationSurreal,
    PatchBranchSurreal, PatchOrganizationSurreal,
};

use common::errors::BasicError;
use common::responses::DeleteResponseResult;
use common::types::BasicResult;
use domain::models::organization::{
    Branch, CreateBranch, CreateOrganization, Organization, PatchBranch, PatchOrganization,
};
use domain::repositories::organization::{
    BranchQueryParams, BranchTrait, OrganizationQueryParams, OrganizationTrait,
};
use domain::repositories::repository::{ResultPaging, ResultPagingDB};

// ==================== ORGANIZATION ==================== //
pub struct OrganizationSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl OrganizationSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        OrganizationSurrealRepository { pool: db }
    }
}

#[async_trait]
impl OrganizationTrait for OrganizationSurrealRepository {
    async fn create(&self, organization: CreateOrganization) -> BasicResult<Organization> {
        let result: Option<GetOrganizationSurreal> = self
            .pool
            .create("organization")
            .content(CreateOrganizationSurreal::from(organization))
            .await?
            .pop();

        match result {
            Some(organization) => Ok(organization.into()),
            None => Err(BasicError::cannot_create_error(String::from(
                "Organization cannot be created!",
            ))),
        }
    }

    async fn delete(&self, current_id: String) -> BasicResult<DeleteResponseResult> {
        let result: Option<GetOrganizationSurreal> = self.pool.delete(("organization", &current_id)).await?;

        self.pool
            .query(
                "DELETE branch WHERE organization_id = $current_id;\
                DELETE relation WHERE organization_id = $current_id;\
                DELETE telegram_group WHERE organization_id = $current_id;\
                DELETE fcm_subsctiption WHERE organization_id = $current_id;\
                DELETE subsctiption WHERE organization_id = $current_id;",
            )
            .bind(("current_id", current_id))
            .await?;

        match result.is_some() {
            true => Ok(DeleteResponseResult { status_code: 204 }),
            false => Err(BasicError::not_found_error(String::from(
                "Organization not found!",
            ))),
        }
    }

    async fn get(&self, current_id: String) -> BasicResult<Organization> {
        let result: Option<GetOrganizationSurreal> = self.pool.select(("organization", current_id)).await?;

        match result {
            Some(organization) => Ok(organization.into()),
            None => Err(BasicError::not_found_error(String::from(
                "Organization not found!",
            ))),
        }
    }

    async fn list(&self, query_params: OrganizationQueryParams) -> BasicResult<ResultPaging<Organization>> {
        let query_data = query_params.query_params();

        let result: Option<ResultPagingDB<GetOrganizationSurreal>> = self
            .pool
            .query(format!(
                "$limit = {}; $offset = {}; $start = {}; $result = SELECT * FROM organization {} START $start LIMIT $limit;\
                RETURN {{total: COUNT(SELECT count() FROM organization), limit: $limit, count: COUNT($result), page: $offset, items: $result}};",
                query_data.limit.unwrap_or_default(),
                query_data.offset.unwrap_or_default(),
                query_data.start.unwrap_or_default(),
                query_data.result.unwrap_or_default(),
            ))
            .await?
            .take(4)?;
        result
            .ok_or(BasicError::bad_request_error(String::from("Bad request!")))
            .map(ResultPagingDB::into)
    }

    async fn patch(&self, id: String, organization: PatchOrganization) -> BasicResult<Organization> {
        let result: Option<GetOrganizationSurreal> = self
            .pool
            .update(("organization", id))
            .merge(PatchOrganizationSurreal::from(organization))
            .await?;

        match result {
            Some(organization) => Ok(organization.into()),
            None => Err(BasicError::not_found_error(String::from(
                "Organization not found!",
            ))),
        }
    }
}

// ==================== BRANCH ==================== //
pub struct BranchSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl BranchSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        BranchSurrealRepository { pool: db }
    }
}

#[async_trait]
impl BranchTrait for BranchSurrealRepository {
    async fn create(&self, branch: CreateBranch) -> BasicResult<Branch> {
        let result: Option<GetBranchSurreal> = self
            .pool
            .create("branch")
            .content(CreateBranchSurreal::from(branch))
            .await?
            .pop();
        match result {
            Some(branch) => Ok(branch.into()),
            None => Err(BasicError::cannot_create_error(String::from(
                "Branch cannot be created!",
            ))),
        }
    }

    async fn delete(&self, current_id: String) -> BasicResult<DeleteResponseResult> {
        let result: Option<GetBranchSurreal> = self.pool.delete(("branch", &current_id)).await?;

        self.pool
            .query(
                "DELETE relation WHERE branch_id = $current_id;\
                DELETE telegram_group WHERE branch_id = $current_id;\
                DELETE fcm_subsctiption WHERE branch_id = $current_id;\
                DELETE subsctiption WHERE branch_id = $current_id;",
            )
            .bind(("current_id", current_id))
            .await?;

        match result.is_some() {
            true => Ok(DeleteResponseResult { status_code: 204 }),
            false => Err(BasicError::not_found_error(String::from("Branch not found!"))),
        }
    }

    async fn get(&self, current_id: String) -> BasicResult<Branch> {
        let result: Option<GetBranchSurreal> = self.pool.select(("branch", current_id)).await?;

        match result {
            Some(branch) => Ok(branch.into()),
            None => Err(BasicError::not_found_error(String::from("Branch not found!"))),
        }
    }

    async fn list(&self, query_params: BranchQueryParams) -> BasicResult<ResultPaging<Branch>> {
        let query_data = query_params.query_params();
        let result: Option<ResultPagingDB<GetBranchSurreal>> = self
            .pool
            .query(format!(
                "$limit = {}; $offset = {}; $start = {}; $result = SELECT * FROM branch {} START $start LIMIT $limit;\
                RETURN {{total: COUNT(SELECT count() FROM branch), limit: $limit, count: COUNT($result), page: $offset, items: $result}};",
                query_data.limit.unwrap_or_default(),
                query_data.offset.unwrap_or_default(),
                query_data.start.unwrap_or_default(),
                query_data.result.unwrap_or_default(),
            ))
            .await?
            .take(4)?;
        result
            .ok_or(BasicError::bad_request_error(String::from("Bad request!")))
            .map(ResultPagingDB::into)
    }

    async fn patch(&self, id: String, branch: PatchBranch) -> BasicResult<Branch> {
        let result: Option<GetBranchSurreal> = self
            .pool
            .update(("branch", id))
            .merge(PatchBranchSurreal::from(branch))
            .await?;
        match result {
            Some(branch) => Ok(branch.into()),
            None => Err(BasicError::not_found_error(String::from("Branch not found!"))),
        }
    }
}
