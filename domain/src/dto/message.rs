use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator_derive::Validate;

use crate::models::message::{CreateKeys, CreateSubscriptionField, FCMSubscription, Subscription, SubscriptionField, TelegramGroup};

// ==================== TelegramBot ==================== //
#[derive(Debug, Serialize, ToSchema, IntoParams)]
pub struct TelegramGroupDTO {
    pub id: String,

    pub group_id: String,

    pub name: Option<String>,

    pub organization_id: String,
    pub branch_id: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<TelegramGroup> for TelegramGroupDTO {
    fn from(value: TelegramGroup) -> Self {
        TelegramGroupDTO {
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

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PostTelegramGroupDTO {
    #[validate(length(min = 1, max = 256, message = "Group's id can't be empty!"))]
    pub group_id: String,

    #[validate(length(min = 1, max = 256, message = "Group's name can't be empty!"))]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PatchTelegramGroupDTO {
    #[validate(length(min = 1, max = 256, message = "Group's id can't be empty!"))]
    pub group_id: Option<String>,

    #[validate(length(min = 1, max = 256, message = "Group's name can't be empty!"))]
    pub name: Option<String>,
}

// ==================== FCMSubscription ==================== //
#[derive(Debug, Serialize, ToSchema, IntoParams)]
pub struct FCMSubscriptionDTO {
    pub id: String,

    pub fcm_token: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: String,
}

impl From<FCMSubscription> for FCMSubscriptionDTO {
    fn from(value: FCMSubscription) -> Self {
        FCMSubscriptionDTO {
            id: value.id,
            fcm_token: value.fcm_token,
            organization_id: value.organization_id,
            branch_id: value.branch_id,
            user_id: value.user_id,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PostFCMSubscriptionDTO {
    #[validate(length(min = 1, max = 256, message = "fcm foken can't be empty!"))]
    pub fcm_token: String,
}

// ==================== Subscription ==================== //
#[derive(Debug, Serialize, ToSchema, IntoParams)]
pub struct SubscriptionDTO {
    pub id: String,

    pub subscription: SubscriptionField,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,

    pub created_at: String,
}

impl From<Subscription> for SubscriptionDTO {
    fn from(value: Subscription) -> Self {
        SubscriptionDTO {
            id: value.id,
            subscription: value.subscription,
            organization_id: value.organization_id,
            branch_id: value.branch_id,
            user_id: value.user_id,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct PostKeysDTO {
    #[validate(length(min = 1, max = 256, message = "p256dh can't be empty!"))]
    pub p256dh: String,
    #[validate(length(min = 1, max = 256, message = "Auth can't be empty!"))]
    pub auth: String,
}

impl From<CreateKeys> for PostKeysDTO {
    fn from(value: CreateKeys) -> Self {
        PostKeysDTO {
            p256dh: value.p256dh,
            auth: value.auth,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, ToSchema, Validate)]
pub struct PostSubscriptionFieldDTO {
    #[validate(length(min = 1, max = 256, message = " can't be empty!"))]
    pub endpoint: String,
    pub expirationTime: Option<String>,
    pub keys: CreateKeys,
}

impl From<PostSubscriptionFieldDTO> for CreateSubscriptionField {
    fn from(value: PostSubscriptionFieldDTO) -> Self {
        CreateSubscriptionField {
            endpoint: value.endpoint,
            expirationTime: value.expirationTime,
            keys: CreateKeys::from(value.keys),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PostSubscriptionDTO {
    pub subscription: PostSubscriptionFieldDTO,
}
