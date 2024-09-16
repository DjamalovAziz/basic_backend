use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::schemas::message::{fcm_subscriptions, subscriptions, telegram_groups};

use domain::models::message::{
    CreateFCMSubscription, CreateSubscription, CreateTelegramGroup, FCMSubscription, PatchTelegramGroup,
    Subscription, SubscriptionField, TelegramGroup,
};

// ==================== TelegramGroup ==================== //

#[derive(Debug, Queryable)]
pub struct GetTelegramGroupDiesel {
    pub id: String,

    pub group_id: String,

    pub name: Option<String>,

    pub organization_id: String,
    pub branch_id: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetTelegramGroupDiesel> for TelegramGroup {
    fn from(value: GetTelegramGroupDiesel) -> Self {
        TelegramGroup {
            id: value.id,

            group_id: value.group_id,

            name: value.name,

            organization_id: value.organization_id,
            branch_id: value.branch_id,

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "telegram_groups"]
pub struct CreateTelegramGroupDiesel {
    pub id: String,

    pub group_id: String,

    pub name: Option<String>,

    pub organization_id: String,
    pub branch_id: String,

    pub created_at: String,
}

impl From<CreateTelegramGroup> for CreateTelegramGroupDiesel {
    fn from(value: CreateTelegramGroup) -> Self {
        CreateTelegramGroupDiesel {
            id: Uuid::new_v4().to_string(),

            group_id: value.group_id,

            name: value.name,

            organization_id: value.organization_id,
            branch_id: value.branch_id,

            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, AsChangeset, Serialize)]
#[table_name = "telegram_groups"]
pub struct PatchTelegramGroupDiesel {
    pub group_id: Option<String>,

    pub name: Option<String>,

    pub updated_at: String,
}

impl From<PatchTelegramGroup> for PatchTelegramGroupDiesel {
    fn from(value: PatchTelegramGroup) -> Self {
        PatchTelegramGroupDiesel {
            group_id: value.group_id,
            name: value.name,
            updated_at: value.updated_at.to_string(),
        }
    }
}

// ==================== FCMSubscription ==================== //
#[derive(Debug, Queryable)]
pub struct GetFCMSubscriptionDiesel {
    pub id: String,

    pub fcm_token: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: String,
}

impl From<GetFCMSubscriptionDiesel> for FCMSubscription {
    fn from(value: GetFCMSubscriptionDiesel) -> Self {
        FCMSubscription {
            id: value.id,
            fcm_token: value.fcm_token,
            organization_id: value.organization_id,
            branch_id: value.branch_id,
            user_id: value.user_id,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "fcm_subscriptions"]
pub struct CreateFCMSubscriptionDiesel {
    pub id: String,

    pub fcm_token: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: String,
}

impl From<CreateFCMSubscription> for CreateFCMSubscriptionDiesel {
    fn from(value: CreateFCMSubscription) -> Self {
        CreateFCMSubscriptionDiesel {
            id: Uuid::new_v4().to_string(),

            fcm_token: value.fcm_token,

            organization_id: value.organization_id,
            branch_id: value.branch_id,
            user_id: value.user_id,

            created_at: value.created_at.to_string(),
        }
    }
}

// ==================== Subscription ==================== //
#[derive(Debug, Queryable)]
pub struct GetSubscriptionDiesel {
    pub id: String,

    pub subscription: SubscriptionField,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: String,
}

impl From<GetSubscriptionDiesel> for Subscription {
    fn from(value: GetSubscriptionDiesel) -> Self {
        Subscription {
            id: value.id,

            subscription: value.subscription,

            organization_id: value.organization_id,
            branch_id: value.branch_id,
            user_id: value.user_id,

            created_at: value.created_at,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Insertable)]
#[table_name = "subscriptions"]
pub struct CreateSubscriptionDiesel {
    pub id: String,

    pub endpoint: String,
    pub expirationTime: Option<String>,
    
    pub p256dh: String,
    pub auth: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: String,
}

impl From<CreateSubscription> for CreateSubscriptionDiesel {
    fn from(value: CreateSubscription) -> Self {
        CreateSubscriptionDiesel {
            id: Uuid::new_v4().to_string(),

            endpoint: value.subscription.endpoint,
            expirationTime: value.subscription.expirationTime,

            p256dh: value.subscription.keys.p256dh,
            auth: value.subscription.keys.auth,

            organization_id: value.organization_id,
            branch_id: value.branch_id,
            user_id: value.user_id,

            created_at: value.created_at.to_string(),
        }
    }
}
