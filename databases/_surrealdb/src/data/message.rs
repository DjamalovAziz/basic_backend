use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::sql::Thing;

use domain::models::message::{
    CreateFCMSubscription, CreateSubscription, CreateSubscriptionField, CreateTelegramGroup, FCMSubscription, PatchTelegramGroup, Subscription, SubscriptionField, TelegramGroup
};

// ==================== TelegramGroup ==================== //

#[derive(Debug, Deserialize)]
pub struct GetTelegramGroupSurreal {
    pub id: Thing,

    pub group_id: String,

    pub name: Option<String>,

    pub organization_id: Thing,
    pub branch_id: Thing,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetTelegramGroupSurreal> for TelegramGroup {
    fn from(value: GetTelegramGroupSurreal) -> Self {
        TelegramGroup {
            id: value.id.id.to_string(),

            group_id: value.group_id,

            name: value.name,

            organization_id: value.organization_id.id.to_string(),
            branch_id: value.branch_id.id.to_string(),

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct CreateTelegramGroupSurreal {
    pub group_id: String,

    pub name: Option<String>,

    pub organization_id: Thing,
    pub branch_id: Thing,

    pub created_at: String,
}

impl From<CreateTelegramGroup> for CreateTelegramGroupSurreal {
    fn from(value: CreateTelegramGroup) -> Self {
        CreateTelegramGroupSurreal {
            group_id: value.group_id,

            name: value.name,

            organization_id: Thing::from(("organization", value.organization_id.as_str())),
            branch_id: Thing::from(("branch", value.branch_id.as_str())),

            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PatchTelegramGroupSurreal {
    pub group_id: Option<String>,

    pub name: Option<String>,

    pub updated_at: String,
}

impl From<PatchTelegramGroup> for PatchTelegramGroupSurreal {
    fn from(value: PatchTelegramGroup) -> Self {
        PatchTelegramGroupSurreal {
            group_id: value.group_id,
            name: value.name,
            updated_at: value.updated_at.to_string(),
        }
    }
}

// ==================== FCMSubscription ==================== //
#[derive(Debug, Deserialize)]
pub struct GetFCMSubscriptionSurreal {
    pub id: Thing,

    pub fcm_token: String,
    pub project_id: String,

    pub organization_id: Thing,
    pub branch_id: Thing,
    pub user_id: Thing,

    pub created_at: String,
}

impl From<GetFCMSubscriptionSurreal> for FCMSubscription {
    fn from(value: GetFCMSubscriptionSurreal) -> Self {
        FCMSubscription {
            id: value.id.id.to_string(),

            fcm_token: value.fcm_token,

            organization_id: value.organization_id.id.to_string(),
            branch_id: value.branch_id.id.to_string(),
            user_id: value.user_id.id.to_string(),

            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateFCMSubscriptionSurreal {
    pub fcm_token: String,

    pub organization_id: Thing,
    pub branch_id: Thing,
    pub user_id: Thing,

    pub created_at: String,
}

impl From<CreateFCMSubscription> for CreateFCMSubscriptionSurreal {
    fn from(value: CreateFCMSubscription) -> Self {
        CreateFCMSubscriptionSurreal {
            fcm_token: value.fcm_token,

            organization_id: Thing::from(("organization", value.organization_id.as_str())),
            branch_id: Thing::from(("branch", value.branch_id.as_str())),
            user_id: Thing::from(("user", value.user_id.as_str())),

            created_at: value.created_at.to_string(),
        }
    }
}

// ==================== Subscription ==================== //
#[derive(Debug, Deserialize)]
pub struct GetSubscriptionSurreal {
    pub id: Thing,

    pub subscription: SubscriptionField,

    pub organization_id: Thing,
    pub branch_id: Thing,
    pub user_id: Thing,

    pub created_at: String,
}

impl From<GetSubscriptionSurreal> for Subscription {
    fn from(value: GetSubscriptionSurreal) -> Self {
        Subscription {
            id: value.id.id.to_string(),

            subscription: value.subscription,

            organization_id: value.organization_id.id.to_string(),
            branch_id: value.branch_id.id.to_string(),
            user_id: value.user_id.id.to_string(),

            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateSubscriptionSurreal {
    pub subscription: CreateSubscriptionField,

    pub organization_id: Thing,
    pub branch_id: Thing,
    pub user_id: Thing,

    pub created_at: String,
}

impl From<CreateSubscription> for CreateSubscriptionSurreal {
    fn from(value: CreateSubscription) -> Self {
        CreateSubscriptionSurreal {
            subscription: CreateSubscriptionField::from(value.subscription),

            organization_id: Thing::from(("organization", value.organization_id.as_str())),
            branch_id: Thing::from(("branch", value.branch_id.as_str())),
            user_id: Thing::from(("user", value.user_id.as_str())),

            created_at: value.created_at.to_string(),
        }
    }
}
