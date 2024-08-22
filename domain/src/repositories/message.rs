use async_trait::async_trait;

use crate::models::message::{
    CreateFCMSubscription, CreateSubscription, CreateTelegramGroup, FCMSubscription, PatchTelegramGroup, Subscription,
    TelegramGroup,
};

use common::responses::DeleteResponseResult;
use common::types::BasicResult;

// ==================== TelegramGroup ==================== //
#[async_trait]
pub trait TelegramGroupTrait: Send + Sync {
    async fn create(&self, camera: CreateTelegramGroup) -> BasicResult<TelegramGroup>;
    async fn delete(&self, current_id: String) -> BasicResult<DeleteResponseResult>;
    async fn get(&self, current_id: String) -> BasicResult<TelegramGroup>;
    async fn list(&self, organization_id: String) -> BasicResult<Vec<TelegramGroup>>;
    async fn patch(&self, current_id: String, camera: PatchTelegramGroup) -> BasicResult<TelegramGroup>;
}

// ==================== FCMSubscription ==================== //

#[async_trait]
pub trait FCMSubscriptionTrait: Send + Sync {
    async fn delete_by_id(&self, fcm_subscription_id: String) -> BasicResult<()>;
    // BASIC
    async fn create(&self, branch: CreateFCMSubscription) -> BasicResult<FCMSubscription>;
}

// ==================== SUBSCRIPTION ==================== //
#[async_trait]
pub trait SubscriptionTrait: Send + Sync {
    async fn delete_by_subscription(&self, subscription: String) -> BasicResult<()>;
    // BASIC
    async fn create(&self, branch: CreateSubscription) -> BasicResult<Subscription>;
}
