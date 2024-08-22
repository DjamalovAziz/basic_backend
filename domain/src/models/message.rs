use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dto::message::PostSubscriptionFieldDTO;

// ==================== TelegramGroup ==================== //

#[derive(Serialize, Debug, Deserialize)]
pub struct TelegramGroup {
    pub id: String,

    pub group_id: String,

    pub name: Option<String>,

    pub organization_id: String,
    pub branch_id: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTelegramGroup {
    pub group_id: String,

    pub name: Option<String>,

    pub organization_id: String,
    pub branch_id: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchTelegramGroup {
    pub group_id: Option<String>,

    pub name: Option<String>,

    pub updated_at: DateTime<Utc>,
}

// ==================== FCMSubscription ==================== //

#[derive(Serialize, Debug, Deserialize)]
pub struct FCMSubscription {
    pub id: String,

    pub fcm_token: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFCMSubscription {
    pub fcm_token: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: DateTime<Utc>,
}

// ==================== Subscription ==================== //
#[derive(Serialize, Debug, Deserialize, ToSchema)]
pub struct Keys {
    pub p256dh: String,
    pub auth: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, ToSchema)]
pub struct SubscriptionField {
    pub endpoint: String,
    pub expirationTime: Option<String>,
    pub keys: Keys,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Subscription {
    pub id: String,

    pub subscription: SubscriptionField,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscription {
    pub subscription: PostSubscriptionFieldDTO,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: DateTime<Utc>,
}
