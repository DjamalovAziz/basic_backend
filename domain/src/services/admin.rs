use async_trait::async_trait;

use common::errors::BasicError;
use common::responses::DeleteResponseResult;

use crate::dto::admin::{
    CreateAdminCLIDTO, PatchAdminCLIDTO, PatchAdminDTO, PostAdminDTO, PutAdminPasswordDTO, SignInAdminDTO,
    SignUpAdminCLIDTO,
};
use crate::models::admin::Admin;
use crate::repositories::admin::AdminQueryParams;
use crate::repositories::repository::{ResultPaging, Token};

// ==================== ADMIN ==================== //
#[async_trait]
pub trait AdminService: Sync + Send {
    async fn change_password(
        &self,
        id: String,
        data: PutAdminPasswordDTO,
        relation_id: String,
    ) -> Result<String, BasicError>;
    async fn create_cli(&self, data: CreateAdminCLIDTO) -> Result<Admin, BasicError>;
    async fn get_me_admin(&self, relation_id: String) -> Result<Admin, BasicError>;
    async fn merge_cli(&self, current_phone_number: String, data: PatchAdminCLIDTO) -> Result<Admin, BasicError>;
    async fn reset_admin_password(&self, phone_number: &str) -> Result<String, BasicError>;
    async fn signin_admin(&self, data: SignInAdminDTO) -> Result<Token, BasicError>;
    async fn signup_cli(&self, data: SignUpAdminCLIDTO) -> Result<Admin, BasicError>;

    // BASIC
    async fn create(&self, data: PostAdminDTO, relation_id: String) -> Result<Admin, BasicError>;
    async fn delete(&self, id: String, relation_id: String) -> Result<DeleteResponseResult, BasicError>;
    async fn get(&self, id: String, relation_id: String) -> Result<Admin, BasicError>;
    async fn list(
        &self,
        query_params: AdminQueryParams,
        relation_id: String,
    ) -> Result<ResultPaging<Admin>, BasicError>;
    async fn patch(&self, id: String, data: PatchAdminDTO, relation_id: String) -> Result<Admin, BasicError>;
}
