use async_trait::async_trait;
use chrono::{SubsecRound, Utc};
use std::sync::Arc;
use validator::Validate;

use common::errors::BasicError;
use common::responses::DeleteResponseResult;
use domain::dto::message::{PatchTelegramGroupDTO, PostFCMSubscriptionDTO, PostSubscriptionDTO, PostTelegramGroupDTO};
use domain::models::message::{
    CreateFCMSubscription, CreateSubscription, CreateTelegramGroup, FCMSubscription, PatchTelegramGroup, Subscription,
    TelegramGroup,
};
use domain::repositories::management::RelationTrait;
use domain::repositories::message::{FCMSubscriptionTrait, SubscriptionTrait, TelegramGroupTrait};
use domain::services::message::{FCMSubscriptionService, SubscriptionService, TelegramGroupService};

use crate::permission::common::{permission_controller, permission_no_branch_id_controller};

// ==================== TelegramGroup ==================== //

pub struct TelegramGroupServiceImpl {
    pub repository: Arc<dyn TelegramGroupTrait>,
    pub relation_repository: Arc<dyn RelationTrait>,
}

impl TelegramGroupServiceImpl {
    pub fn new(repository: Arc<dyn TelegramGroupTrait>, relation_repository: Arc<dyn RelationTrait>) -> Self {
        TelegramGroupServiceImpl {
            repository,
            relation_repository,
        }
    }
}

#[async_trait]
impl TelegramGroupService for TelegramGroupServiceImpl {
    async fn create(
        &self,
        data: PostTelegramGroupDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<TelegramGroup, BasicError> {
        permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .create(CreateTelegramGroup {
                        group_id: data.group_id,
                        name: data.name,
                        organization_id,
                        branch_id,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn delete(
        &self,
        current_id: String,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<DeleteResponseResult, BasicError> {
        permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
        )
        .await?;

        self.repository.delete(current_id).await
    }

    async fn get(
        &self,
        current_id: String,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<TelegramGroup, BasicError> {
        permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
        )
        .await?;

        self.repository.get(current_id).await
    }

    async fn list(&self, user_id: String, organization_id: String) -> Result<Vec<TelegramGroup>, BasicError> {
        permission_no_branch_id_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
        )
        .await?;

        self.repository.list(organization_id).await
    }

    async fn patch(
        &self,
        current_id: String,
        data: PatchTelegramGroupDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<TelegramGroup, BasicError> {
        permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .patch(
                        current_id,
                        PatchTelegramGroup {
                            group_id: data.group_id,
                            name: data.name,
                            updated_at: Utc::now().trunc_subsecs(0),
                        },
                    )
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }
}

// ==================== FCMSUBSCRIPTION ==================== //
pub struct FCMSubscriptionServiceImpl {
    pub repository: Arc<dyn FCMSubscriptionTrait>,
    pub relation_repository: Arc<dyn RelationTrait>,
}

impl FCMSubscriptionServiceImpl {
    pub fn new(repository: Arc<dyn FCMSubscriptionTrait>, relation_repository: Arc<dyn RelationTrait>) -> Self {
        FCMSubscriptionServiceImpl {
            repository,
            relation_repository,
        }
    }
}

#[async_trait]
impl FCMSubscriptionService for FCMSubscriptionServiceImpl {
    async fn create(
        &self,
        data: PostFCMSubscriptionDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<FCMSubscription, BasicError> {
        permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .create(CreateFCMSubscription {
                        fcm_token: data.fcm_token,
                        organization_id,
                        branch_id,
                        user_id,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }
}

// ==================== SUBSCRIPTION ==================== //
pub struct SubscriptionServiceImpl {
    pub repository: Arc<dyn SubscriptionTrait>,
    pub relation_repository: Arc<dyn RelationTrait>,
}

impl SubscriptionServiceImpl {
    pub fn new(repository: Arc<dyn SubscriptionTrait>, relation_repository: Arc<dyn RelationTrait>) -> Self {
        SubscriptionServiceImpl {
            repository,
            relation_repository,
        }
    }
}

#[async_trait]
impl SubscriptionService for SubscriptionServiceImpl {
    async fn create(
        &self,
        data: PostSubscriptionDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<Subscription, BasicError> {
        permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &branch_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .create(CreateSubscription {
                        subscription: data.subscription,
                        organization_id,
                        branch_id,
                        user_id,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }
}
