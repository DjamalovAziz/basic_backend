use async_trait::async_trait;
use common::enums::{RelationType, UserRoleType};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::models::management::{
    CreateRelation, CreateUser, PatchInviteToBranch, PatchRelation, PatchUser, PutUserPassword,
};
use crate::repositories::repository::ResultPaging;

use crate::models::management::{Relation, User};
use common::responses::DeleteResponseResult;
use common::types::BasicResult;

// ==================== USER ==================== //
#[derive(Default, Debug, Serialize, Deserialize, IntoParams)]
pub struct UserQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start: Option<u32>,

    pub created_at_from: Option<String>,
    pub created_at_to: Option<String>,

    pub updated_at_from: Option<String>,
    pub updated_at_to: Option<String>,

    pub result: Option<String>,
}

#[async_trait]
pub trait UserTrait: Send + Sync {
    async fn get_by_phone_number(&self, phone_number: &str) -> BasicResult<User>;
    async fn get_user_by_id(&self, id: &str) -> BasicResult<User>;
    //
    async fn change_password(&self, id: String, data: PutUserPassword) -> BasicResult<String>;
    //
    async fn create(&self, user: CreateUser) -> BasicResult<User>;
    async fn get(&self, current_id: String) -> BasicResult<User>;
    async fn list(&self, query_params: UserQueryParams) -> BasicResult<ResultPaging<User>>;
    //
    async fn delete_self(&self, self_id: String) -> BasicResult<DeleteResponseResult>;
    async fn get_self(&self, self_id: String) -> BasicResult<User>;
    async fn patch_self(&self, self_id: String, relation: PatchUser) -> BasicResult<User>;
}

// ==================== Relation ==================== //

#[derive(Default, Debug, Serialize, Deserialize, IntoParams)]
pub struct RelationQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start: Option<u32>,

    pub role: Option<UserRoleType>,

    pub relation_type: Option<RelationType>,

    pub organization_id: Option<String>,
    pub branch_id: Option<String>,

    pub created_at_from: Option<String>,
    pub created_at_to: Option<String>,

    pub updated_at_from: Option<String>,
    pub updated_at_to: Option<String>,

    pub result: Option<String>,
}

#[async_trait]
pub trait RelationTrait: Send + Sync {
    async fn get_relations_by_user_id(&self, user_id: &str) -> BasicResult<Vec<Relation>>;
    //
    async fn create(&self, relation: CreateRelation) -> BasicResult<Relation>;
    async fn delete(&self, current_id: String) -> BasicResult<DeleteResponseResult>;
    async fn list_my_relations(&self, user_id: String) -> BasicResult<Vec<Relation>>;
    async fn patch_invitation_to_branch(&self, id: String, relation: PatchInviteToBranch) -> BasicResult<Relation>;
    async fn patch(&self, id: String, relation: PatchRelation) -> BasicResult<Relation>;
    async fn list(&self, params: RelationQueryParams) -> BasicResult<ResultPaging<Relation>>;
}
