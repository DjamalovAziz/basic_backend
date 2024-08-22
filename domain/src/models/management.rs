use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::enums::{RelationType, UserRoleType};

use crate::dto::management::{PatchUserDTO, PostUserDTO};

// ==================== USER ==================== //

#[derive(Debug, Serialize, Deserialize)]
pub struct PutUserPassword {
    pub password: String,

    pub updated_at: DateTime<Utc>,
}
//

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct User {
    pub id: String,

    pub password: String,
    pub image_url: String,
    pub phone_number: String,

    pub email: Option<String>,

    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateUser {
    pub password: String,
    pub image_path: String,
    pub phone_number: String,

    pub email: Option<String>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ServicePostUserImage {
    pub user: PostUserDTO,

    pub image_destination: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchUser {
    pub image_path: Option<String>,
    pub phone_number: Option<String>,

    pub email: Option<String>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ServicePatchUserImage {
    pub user: PatchUserDTO,

    pub image_destination: Option<String>,
}

// ==================== Relation ==================== //

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct Relation {
    pub id: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,
    pub role: UserRoleType,
    pub relation_type: RelationType,

    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRelation {
    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,
    pub role: UserRoleType,
    pub relation_type: RelationType,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchInviteToBranch {
    pub user_id: Option<String>,
    pub role: Option<UserRoleType>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchRelation {
    pub role: Option<UserRoleType>,

    pub updated_at: DateTime<Utc>,
}
