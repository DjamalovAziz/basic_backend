use async_trait::async_trait;

use crate::dto::management::{
    InviteToBranchDTO, PatchInviteToBranchDTO, PatchRelationDTO, PutUserPasswordDTO, RequestJoinToBranchDTO,
    SignInUserDTO, SignUpDTO,
};
use crate::models::management::{Relation, User};
use crate::models::management::{ServicePatchUserImage, ServicePostUserImage};
use crate::repositories::management::{RelationQueryParams, UserQueryParams};
use crate::repositories::repository::{ResultPaging, Token};

use common::errors::BasicError;
use common::responses::DeleteResponseResult;

// ==================== USER ==================== //
#[async_trait]
pub trait UserService: Sync + Send {
    async fn change_password(&self, data: PutUserPasswordDTO, relation_id: String) -> Result<String, BasicError>;
    async fn reset_user_password(&self, data: &str) -> Result<String, BasicError>;
    async fn signin_user(&self, data: SignInUserDTO) -> Result<Token, BasicError>;
    async fn signup(&self, data: SignUpDTO) -> Result<Token, BasicError>;
    //
    async fn create(&self, form_data: ServicePostUserImage, relation_id: String) -> Result<User, BasicError>;
    async fn get(&self, id: String, user_id: String) -> Result<User, BasicError>;
    async fn list(&self, query_params: UserQueryParams, user_id: String) -> Result<ResultPaging<User>, BasicError>;
    //
    async fn delete_self(&self, self_id: String) -> Result<DeleteResponseResult, BasicError>;
    async fn get_self(&self, self_id: String) -> Result<User, BasicError>;
    async fn patch_self(&self, data: ServicePatchUserImage, self_id: String) -> Result<User, BasicError>;
}

// ==================== Relation ==================== //

#[async_trait]
pub trait RelationService: Sync + Send {
    async fn delete(
        &self,
        current_id: String,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<DeleteResponseResult, BasicError>;
    async fn invite_to_branch(
        &self,
        relation: InviteToBranchDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<Relation, BasicError>;
    async fn list_my_relations(&self, user_id: String) -> Result<Vec<Relation>, BasicError>;
    async fn list(&self, params: RelationQueryParams, user_id: String) -> Result<ResultPaging<Relation>, BasicError>;
    async fn patch(
        &self,
        current_id: String,
        data: PatchRelationDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<Relation, BasicError>;
    async fn patch_invitation_to_branch(
        &self,
        current_id: String,
        data: PatchInviteToBranchDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<Relation, BasicError>;
    async fn request_join_to_branch(
        &self,
        relation: RequestJoinToBranchDTO,
        user_id: String,
    ) -> Result<Relation, BasicError>;
}
