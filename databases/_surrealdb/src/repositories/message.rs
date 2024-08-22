use std::sync::Arc;

use async_trait::async_trait;
use common::responses::DeleteResponseResult;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use common::errors::BasicError;
use common::types::BasicResult;
use domain::models::message::{CreateFCMSubscription, CreateSubscription, CreateTelegramGroup, PatchTelegramGroup};
use domain::models::message::{FCMSubscription, Subscription, TelegramGroup};
use domain::repositories::message::{FCMSubscriptionTrait, SubscriptionTrait, TelegramGroupTrait};

use crate::data::message::{
    CreateFCMSubscriptionSurreal, CreateSubscriptionSurreal, CreateTelegramGroupSurreal, GetFCMSubscriptionSurreal,
    GetSubscriptionSurreal, GetTelegramGroupSurreal, PatchTelegramGroupSurreal,
};

// ==================== CAMERA ==================== //
pub struct TelegramGroupSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl TelegramGroupSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        TelegramGroupSurrealRepository { pool: db }
    }
}

#[async_trait]
impl TelegramGroupTrait for TelegramGroupSurrealRepository {
    async fn create(&self, telegram_group: CreateTelegramGroup) -> BasicResult<TelegramGroup> {
        let result: Option<GetTelegramGroupSurreal> = self
            .pool
            .create("telegram_group")
            .content(CreateTelegramGroupSurreal::from(telegram_group))
            .await?
            .pop();

        match result {
            Some(telegram_group_surreal) => Ok(telegram_group_surreal.into()),
            None => Err(BasicError::cannot_create_error(String::from(
                "Telegram Group cannot be created!",
            ))),
        }
    }

    async fn delete(&self, current_id: String) -> BasicResult<DeleteResponseResult> {
        let result: Option<GetTelegramGroupSurreal> = self.pool.delete(("telegram_group", current_id)).await?;

        match result.is_some() {
            true => Ok(DeleteResponseResult { status_code: 204 }),
            false => Err(BasicError::not_found_error(String::from("Telegram Group not found!"))),
        }
    }

    async fn get(&self, current_id: String) -> BasicResult<TelegramGroup> {
        let result: Option<GetTelegramGroupSurreal> = self.pool.select(("telegram_group", current_id)).await?;

        match result {
            Some(telegram_group) => Ok(telegram_group.into()),
            None => Err(BasicError::not_found_error(String::from("Telegram Group not found!"))),
        }
    }

    async fn list(&self, organization_id: String) -> BasicResult<Vec<TelegramGroup>> {
        let result: Vec<GetTelegramGroupSurreal> = self
            .pool
            .query("SELECT * FROM telegram_group WHERE organization_id = $organization_id")
            .bind(("organization_id", organization_id))
            .await?
            .take(0)?;

        let telegram_groups: Vec<TelegramGroup> = result.into_iter().map(TelegramGroup::from).collect();

        Ok(telegram_groups)
    }

    async fn patch(&self, id: String, telegram_group: PatchTelegramGroup) -> BasicResult<TelegramGroup> {
        let result: Option<GetTelegramGroupSurreal> = self
            .pool
            .update(("telegram_group", id))
            .merge(PatchTelegramGroupSurreal::from(telegram_group))
            .await?;
        match result {
            Some(telegram_group) => Ok(telegram_group.into()),
            None => Err(BasicError::not_found_error(String::from("Telegram Group not found!"))),
        }
    }
}

// ==================== FCMSUBSCRIPTION ==================== //

pub struct FCMSubscriptionSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl FCMSubscriptionSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        FCMSubscriptionSurrealRepository { pool: db }
    }
}

#[async_trait]
impl FCMSubscriptionTrait for FCMSubscriptionSurrealRepository {
    async fn delete_by_id(&self, fcm_subscription_id: String) -> BasicResult<()> {
        let result: Option<GetFCMSubscriptionSurreal> = self
            .pool
            .query("DELETE fcm_subscription WHERE id = fcm_subscription:fcm_subscription_id")
            .bind(("fcm_subscription_id", fcm_subscription_id))
            .await?
            .take(0)?;

        match result.is_none() {
            true => Ok(()),
            false => Err(BasicError::not_found_error(String::from("FcmSubscription not found!"))),
        }
    }

    // BASIC
    async fn create(&self, fcm_subscription: CreateFCMSubscription) -> BasicResult<FCMSubscription> {
        let result: Option<GetFCMSubscriptionSurreal> = self
            .pool
            .query(format!(
                "SELECT * FROM fcm_subscription WHERE user_id = user:{} && fcm_token == '{}';",
                fcm_subscription.user_id, fcm_subscription.fcm_token
            ))
            .await?
            .take(0)?;
        match result {
            Some(fcm_subscription) => Ok(fcm_subscription.into()),
            None => {
                let result: Option<GetFCMSubscriptionSurreal> = self
                    .pool
                    .create("fcm_subscription")
                    .content(CreateFCMSubscriptionSurreal::from(fcm_subscription))
                    .await?
                    .pop();
                match result {
                    Some(fcm_subscription) => Ok(fcm_subscription.into()),
                    None => Err(BasicError::not_found_error(String::from("FcmSubsctiption not found!"))),
                }
            }
        }
    }
}

// ==================== SUBSCRIPTION ==================== //

pub struct SubscriptionSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl SubscriptionSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        SubscriptionSurrealRepository { pool: db }
    }
}

#[async_trait]
impl SubscriptionTrait for SubscriptionSurrealRepository {
    async fn delete_by_subscription(&self, subscription_id: String) -> BasicResult<()> {
        let result: Option<GetSubscriptionSurreal> = self
            .pool
            .query("DELETE subscription WHERE id = subscription:subscription_id")
            .bind(("subscription_id", subscription_id))
            .await?
            .take(0)?;

        match result.is_none() {
            true => Ok(()),
            false => Err(BasicError::not_found_error(String::from("FcmSubsctiption not found!"))),
        }
    }

    // BASIC
    async fn create(&self, subscription: CreateSubscription) -> BasicResult<Subscription> {
        let result: Option<GetSubscriptionSurreal> = self
            .pool
            .create("subscription")
            .content(CreateSubscriptionSurreal::from(subscription))
            .await?
            .pop();
        match result {
            Some(subscription) => Ok(subscription.into()),
            None => Err(BasicError::cannot_create_error(String::from(
                "Subsctiption cannot be created!",
            ))),
        }
    }
}
