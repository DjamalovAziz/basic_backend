use async_trait::async_trait;

use crate::dto::message::{PatchTelegramGroupDTO, PostFCMSubscriptionDTO, PostSubscriptionDTO, PostTelegramGroupDTO};
use crate::models::message::{FCMSubscription, Subscription, TelegramGroup};
use common::errors::BasicError;
use common::responses::DeleteResponseResult;

// ==================== TelegramGroup ==================== //
#[async_trait]
pub trait TelegramGroupService: Sync + Send {
    // BASIC
    async fn create(
        &self,
        data: PostTelegramGroupDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<TelegramGroup, BasicError>;
    async fn delete(
        &self,
        current_id: String,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<DeleteResponseResult, BasicError>;
    async fn get(
        &self,
        current_id: String,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<TelegramGroup, BasicError>;
    async fn list(&self, user_id: String, organization_id: String) -> Result<Vec<TelegramGroup>, BasicError>;
    async fn patch(
        &self,
        id: String,
        data: PatchTelegramGroupDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<TelegramGroup, BasicError>;
}

// ==================== FCMSUBSCRIPTION ==================== //

#[async_trait]
pub trait FCMSubscriptionService: Sync + Send {
    // BASIC
    async fn create(
        &self,
        branch: PostFCMSubscriptionDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<FCMSubscription, BasicError>;
}

// ==================== SUBSCRIPTION ==================== //

#[async_trait]
pub trait SubscriptionService: Sync + Send {
    // BASIC
    async fn create(
        &self,
        branch: PostSubscriptionDTO,
        user_id: String,
        organization_id: String,
        branch_id: String,
    ) -> Result<Subscription, BasicError>;
}
