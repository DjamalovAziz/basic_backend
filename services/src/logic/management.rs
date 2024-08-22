use async_trait::async_trait;
use chrono::{SubsecRound, Utc};
use common::enums::{RelationType, UserRoleType};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::sync::Arc;
use validator::Validate;

use common::constant::DEFAULT_AVATAR_PATH;
use common::errors::BasicError;
use common::functions::{generate_hash, generate_token, send_sms, verify_password_hash};
use common::responses::DeleteResponseResult;
use domain::dto::management::{
    InviteToBranchDTO, PatchInviteToBranchDTO, PatchRelationDTO, PutUserPasswordDTO, RequestJoinToBranchDTO,
    SignInUserDTO, SignUpDTO,
};
use domain::models::management::{
    CreateRelation, CreateUser, PatchInviteToBranch, PatchRelation, PatchUser, PutUserPassword, ServicePatchUserImage,
    ServicePostUserImage,
};
use domain::models::management::{Relation, User};
use domain::repositories::management::{RelationQueryParams, RelationTrait, UserQueryParams, UserTrait};
use domain::repositories::repository::{ResultPaging, Token};
use domain::services::management::{RelationService, UserService};

use crate::permission::common::permission_controller;
use crate::permission::management::relation_id_permission_controller;

// ==================== USER ==================== //
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserTrait>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserTrait>) -> Self {
        UserServiceImpl { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn change_password(&self, data: PutUserPasswordDTO, user_id: String) -> Result<String, BasicError> {
        match data.validate() {
            Ok(_) => {
                if verify_password_hash(
                    data.actual_password,
                    self.repository.get_user_by_id(&user_id).await?.password,
                )? {
                    self.repository
                        .change_password(
                            user_id,
                            PutUserPassword {
                                password: generate_hash(data.password.as_bytes()).unwrap_or_default(),
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

    async fn reset_user_password(&self, phone_number: &str) -> Result<String, BasicError> {
        let user = self.repository.get_by_phone_number(phone_number).await?;

        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();

        let password: String = (0..10)
            .map(|_| *chars.choose(&mut thread_rng()).expect("reset_user_password"))
            .collect();

        match send_sms(
            user.phone_number.as_str(),
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
                            user.id,
                            PutUserPassword {
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

    async fn signin_user(&self, data: SignInUserDTO) -> Result<Token, BasicError> {
        let user = self.repository.get_by_phone_number(&data.phone_number).await?;

        if verify_password_hash(data.password, user.password)? {
            let current_token = Token {
                access_token: generate_token(user.id).await?,
            };

            Ok(current_token)
        } else {
            Err(BasicError::forbidden_error(String::from("Password is not correct!")))
        }
    }

    async fn signup(&self, data: SignUpDTO) -> Result<Token, BasicError> {
        match data.validate() {
            Ok(_) => {
                let hashed_password = generate_hash(data.password.as_bytes()).unwrap_or_default();

                let user = self
                    .repository
                    .create(CreateUser {
                        password: hashed_password,
                        image_path: DEFAULT_AVATAR_PATH.to_string(),
                        phone_number: data.phone_number,
                        email: data.email,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await?;

                Ok(Token {
                    access_token: generate_token(user.id).await?,
                })
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    //
    async fn create(&self, form_data: ServicePostUserImage, _user_id: String) -> Result<User, BasicError> {
        let user_data = form_data.user;
        match user_data.validate() {
            Ok(_) => {
                let image_path = form_data.image_destination;

                self.repository
                    .create(CreateUser {
                        password: generate_hash(user_data.password.as_bytes()).unwrap_or_default(),
                        phone_number: user_data.phone_number,
                        image_path: image_path.unwrap_or(DEFAULT_AVATAR_PATH.to_string()),

                        email: user_data.email,

                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn get(&self, id: String, _user_id: String) -> Result<User, BasicError> {
        self.repository.get(id).await
    }

    async fn list(&self, query_params: UserQueryParams, _user_id: String) -> Result<ResultPaging<User>, BasicError> {
        self.repository.list(query_params).await
    }

    //

    async fn delete_self(&self, self_id: String) -> Result<DeleteResponseResult, BasicError> {
        self.repository.delete_self(self_id).await
    }

    async fn get_self(&self, self_id: String) -> Result<User, BasicError> {
        self.repository.get(self_id).await
    }

    async fn patch_self(&self, form_data: ServicePatchUserImage, self_id: String) -> Result<User, BasicError> {
        let user_data = form_data.user;
        match user_data.validate() {
            Ok(_) => {
                let image_path = form_data.image_destination;

                self.repository
                    .patch_self(
                        self_id,
                        PatchUser {
                            phone_number: user_data.phone_number,
                            image_path,
                            email: user_data.email,
                            updated_at: Utc::now().trunc_subsecs(0),
                        },
                    )
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }
}

// ==================== Relation ==================== //
pub struct RelationServiceImpl {
    pub repository: Arc<dyn RelationTrait>,
    pub user_repository: Arc<dyn UserTrait>,
}

impl RelationServiceImpl {
    pub fn new(repository: Arc<dyn RelationTrait>, user_repository: Arc<dyn UserTrait>) -> Self {
        RelationServiceImpl {
            repository,
            user_repository,
        }
    }
}

#[async_trait]
impl RelationService for RelationServiceImpl {
    async fn delete(
        &self,
        current_id: String,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<DeleteResponseResult, BasicError> {
        relation_id_permission_controller(
            self.repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
            &current_id,
        )
        .await?;

        self.repository.delete(current_id).await
    }

    async fn invite_to_branch(
        &self,
        data: InviteToBranchDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<Relation, BasicError> {
        permission_controller(
            self.repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .create(CreateRelation {
                        organization_id,
                        branch_id,
                        user_id: data.user_id,
                        role: data.role,
                        relation_type: RelationType::InvitationToUser,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn list_my_relations(&self, user_id: String) -> Result<Vec<Relation>, BasicError> {
        self.repository.list_my_relations(user_id).await
    }

    async fn patch(
        &self,
        current_id: String,
        data: PatchRelationDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<Relation, BasicError> {
        relation_id_permission_controller(
            self.repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
            &current_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .patch(
                        current_id,
                        PatchRelation {
                            role: data.role,
                            updated_at: Utc::now().trunc_subsecs(0),
                        },
                    )
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn patch_invitation_to_branch(
        &self,
        current_id: String,
        data: PatchInviteToBranchDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<Relation, BasicError> {
        permission_controller(
            self.repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .patch_invitation_to_branch(
                        current_id,
                        PatchInviteToBranch {
                            user_id: data.user_id,
                            role: data.role,
                            updated_at: Utc::now().trunc_subsecs(0),
                        },
                    )
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn request_join_to_branch(
        &self,
        data: RequestJoinToBranchDTO,
        user_id: String,
    ) -> Result<Relation, BasicError> {
        self.user_repository.get_user_by_id(&user_id).await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .create(CreateRelation {
                        organization_id: data.organization_id,
                        branch_id: data.branch_id,
                        user_id,
                        role: UserRoleType::Member,
                        relation_type: RelationType::RequestToJoin,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn list(
        &self,
        query_params: RelationQueryParams,
        _user_id: String,
    ) -> Result<ResultPaging<Relation>, BasicError> {
        self.repository.list(query_params).await
    }
}
