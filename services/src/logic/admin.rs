use async_trait::async_trait;
use chrono::{SubsecRound, Utc};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::sync::Arc;
use validator::Validate;

use common::errors::BasicError;
use common::functions::{generate_hash, generate_token, send_sms, verify_password_hash};
use common::responses::DeleteResponseResult;
use domain::dto::admin::{
    CreateAdminCLIDTO, PatchAdminCLIDTO, PatchAdminDTO, PostAdminDTO, PutAdminPasswordDTO, SignInAdminDTO,
    SignUpAdminCLIDTO,
};
use domain::models::admin::{Admin, CreateAdmin, PatchAdmin, PutAdminPassword};
use domain::repositories::admin::{AdminQueryParams, AdminTrait};
use domain::repositories::repository::{ResultPaging, Token};
use domain::services::admin::AdminService;

use crate::permission::admin::{
    admin_get_list_create_permission_controller, admin_get_me_admin_permission_controller, admin_permission_controller,
};

// ==================== ADMIN ==================== //
pub struct AdminServiceImpl {
    pub repository: Arc<dyn AdminTrait>,
}

impl AdminServiceImpl {
    pub fn new(repository: Arc<dyn AdminTrait>) -> Self {
        AdminServiceImpl { repository }
    }
}

#[async_trait]
impl AdminService for AdminServiceImpl {
    async fn change_password(
        &self,
        id: String,
        data: PutAdminPasswordDTO,
        relation_id: String,
    ) -> Result<String, BasicError> {
        match data.validate() {
            Ok(_) => {
                admin_permission_controller(&id, self.repository.get(&relation_id).await?).await?;

                if verify_password_hash(data.actual_password, self.repository.get(&id).await?.password)? {
                    self.repository
                        .change_password(
                            id,
                            PutAdminPassword {
                                password: data.password,
                                updated_at: Utc::now().trunc_subsecs(0),
                            },
                        )
                        .await
                } else {
                    Err(BasicError::forbidden_error(String::from("Password is not correct!")))
                }
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn create_cli(&self, data: CreateAdminCLIDTO) -> Result<Admin, BasicError> {
        self.repository.create(CreateAdmin::from(data)).await
    }

    async fn get_me_admin(&self, relation_id: String) -> Result<Admin, BasicError> {
        admin_get_me_admin_permission_controller(self.repository.get(&relation_id).await?).await?;

        self.repository.get(&relation_id).await
    }

    async fn merge_cli(&self, current_phone_number: String, data: PatchAdminCLIDTO) -> Result<Admin, BasicError> {
        match data.validate() {
            Ok(_) => {
                self.repository
                    .merge_cli(current_phone_number, PatchAdmin::from(data))
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn reset_admin_password(&self, phone_number: &str) -> Result<String, BasicError> {
        let admin = self.repository.get_by_phone_number(phone_number).await?;

        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();

        let password: String = (0..10)
            .map(|_| *chars.choose(&mut thread_rng()).expect("reset_admin_password"))
            .collect();

        match send_sms(
            admin.phone_number.as_str(),
            format!(
                "\n Do not share this message with anyone!!! \n Your new password to login is: {}",
                password
            ),
        )
        .await
        {
            Ok(variaty) => match variaty {
                true => {
                    self.repository
                        .change_password(
                            admin.id,
                            PutAdminPassword {
                                password: generate_hash(password.as_bytes()).unwrap_or_default(),
                                updated_at: Utc::now().trunc_subsecs(0),
                            },
                        )
                        .await
                }
                false => Err(BasicError::bad_request_error(String::from("Error verifying OTP!"))),
            },
            Err(_) => Err(BasicError::bad_request_error(String::from("Error sending OTP!"))),
        }
    }

    async fn signin_admin(&self, data: SignInAdminDTO) -> Result<Token, BasicError> {
        let admin = self.repository.get_by_phone_number(&data.phone_number).await?;

        if verify_password_hash(data.password, admin.password)? {
            let current_token = Token {
                access_token: generate_token(admin.id).await?,
            };

            Ok(current_token)
        } else {
            Err(BasicError::forbidden_error(String::from("Password is not correct!")))
        }
    }

    async fn signup_cli(&self, data: SignUpAdminCLIDTO) -> Result<Admin, BasicError> {
        if !self.repository.is_superadmin_in_db().await? {
            self.repository.create(CreateAdmin::from(data)).await
        } else {
            Err(BasicError::forbidden_error(String::from(
                "Super Admin already signuppded!",
            )))
        }
    }

    //
    async fn delete(&self, id: String, relation_id: String) -> Result<DeleteResponseResult, BasicError> {
        admin_permission_controller(&id, self.repository.get(&relation_id).await?).await?;

        self.repository.delete(id).await
    }

    async fn get(&self, id: String, relation_id: String) -> Result<Admin, BasicError> {
        admin_permission_controller(&id, self.repository.get(&relation_id).await?).await?;

        self.repository.get(&id).await
    }

    async fn list(
        &self,
        query_params: AdminQueryParams,
        relation_id: String,
    ) -> Result<ResultPaging<Admin>, BasicError> {
        admin_get_list_create_permission_controller(self.repository.get(&relation_id).await?).await?;

        self.repository.list(query_params).await
    }

    async fn patch(&self, id: String, data: PatchAdminDTO, relation_id: String) -> Result<Admin, BasicError> {
        match data.validate() {
            Ok(_) => {
                admin_permission_controller(&id, self.repository.get(&relation_id).await?).await?;

                self.repository
                    .patch(
                        id,
                        PatchAdmin {
                            phone_number: data.phone_number,
                            role: data.role,
                            updated_at: Utc::now().trunc_subsecs(0),
                        },
                    )
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn create(&self, data: PostAdminDTO, relation_id: String) -> Result<Admin, BasicError> {
        match data.validate() {
            Ok(_) => {
                admin_get_list_create_permission_controller(self.repository.get(&relation_id).await?).await?;

                self.repository
                    .create(CreateAdmin {
                        password: generate_hash(data.password.as_bytes()).unwrap_or_default(),
                        phone_number: data.phone_number,
                        role: data.role,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }
}
