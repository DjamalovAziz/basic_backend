use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator_derive::Validate;

use common::enums::AdminRoleType;

use crate::models::admin::Admin;

// ==================== ADMIN ==================== //

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct SignInAdminDTO {
    pub phone_number: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PutAdminPasswordDTO {
    #[validate(length(min = 1, max = 256, message = "Actual password can't be empty!"))]
    pub actual_password: String,
    #[validate(length(min = 1, max = 256, message = "Password can't be empty!"))]
    pub password: String,
    #[validate(length(min = 1, max = 256, message = "Second password can't be empty!"))]
    #[validate(must_match(other = "password", message = "Passwords must be match"))]
    pub confirm_password: String,
}

// BASIC
#[derive(Debug, Serialize, ToSchema, IntoParams)]
pub struct AdminDTO {
    pub id: String,

    pub phone_number: String,
    pub role: AdminRoleType,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<Admin> for AdminDTO {
    fn from(value: Admin) -> Self {
        AdminDTO {
            id: value.id,
            phone_number: value.phone_number,
            role: value.role,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Validate)]
pub struct SignUpAdminCLIDTO {
    #[validate(length(min = 1, max = 256, message = "Password can't be empty!"))]
    pub password: String,
    #[validate(length(min = 1, max = 256, message = "Second password can't be empty!"))]
    #[validate(must_match(other = "password", message = "Passwords must be match"))]
    pub confirm_password: String,
    #[validate(length(
        min = 9,
        max = 256,
        message = "The phone number must contain more than 9 characters!"
    ))]
    pub phone_number: String,
}

#[derive(Debug, Validate)]
pub struct CreateAdminCLIDTO {
    pub password: String,
    pub confirm_password: String,
    pub role: AdminRoleType,
    pub phone_number: String,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PostAdminDTO {
    #[validate(length(min = 1, max = 256, message = "Password can't be empty!"))]
    pub password: String,
    #[validate(length(min = 1, max = 256, message = "Second password can't be empty!"))]
    #[validate(must_match(other = "password", message = "Passwords must be match"))]
    pub confirm_password: String,
    pub role: AdminRoleType,
    #[validate(length(
        min = 9,
        max = 256,
        message = "The phone number must contain more than 9 characters!"
    ))]
    pub phone_number: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PatchAdminCLIDTO {
    pub role: Option<AdminRoleType>,
    #[validate(length(
        min = 9,
        max = 256,
        message = "The phone number must contain more than 9 characters!"
    ))]
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PatchAdminDTO {
    pub role: Option<AdminRoleType>,
    #[validate(length(
        min = 9,
        max = 256,
        message = "The phone number must contain more than 9 characters!"
    ))]
    pub phone_number: Option<String>,
}
