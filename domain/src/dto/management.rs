use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator_derive::Validate;

use common::enums::{RelationType, UserRoleType};

use crate::models::management::{Relation, User};

#[derive(Debug, Deserialize, ToSchema)]
pub struct PhoneNumberDTO {
    pub phone_number: String,
}

// ==================== USER ==================== //

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PutUserPasswordDTO {
    #[validate(length(min = 1, max = 256, message = "Actual password can't be empty!"))]
    pub actual_password: String,
    #[validate(length(min = 1, max = 256, message = "Password can't be empty!"))]
    pub password: String,
    #[validate(length(min = 1, max = 256, message = "Second password can't be empty!"))]
    #[validate(must_match(other = "password", message = "Passwords must be match"))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct SignUpDTO {
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

    #[validate(length(min = 1, max = 256, message = "Email name can't be empty!"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SignInUserDTO {
    pub phone_number: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema, IntoParams)]
pub struct UserDTO {
    pub id: String,

    pub image_url: String,
    pub phone_number: String,

    pub email: Option<String>,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        UserDTO {
            id: value.id,
            image_url: value.image_url,
            phone_number: value.phone_number,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PostUserDTO {
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

    #[validate(length(min = 1, max = 256, message = "Email address can't be empty!"))]
    #[validate(email(message = "This email address is not correct!"))]
    pub email: Option<String>,

    pub role: UserRoleType,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct MultipartPostUserImageDTO {
    pub user: PostUserDTO,

    pub image: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PatchUserDTO {
    #[validate(length(
        min = 9,
        max = 256,
        message = "The phone number must contain more than 9 characters!"
    ))]
    pub phone_number: Option<String>,

    #[validate(length(min = 1, max = 256, message = "Email address can't be empty!"))]
    #[validate(email(message = "This email address is not correct!"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct MultipartPatchUserImageDTO {
    pub user: PatchUserDTO,

    pub image: Option<Vec<u8>>,
}

// ==================== Relation ==================== //

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct RelationDTO {
    pub id: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,
    pub role: UserRoleType,
    pub relation_type: RelationType,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<Relation> for RelationDTO {
    fn from(value: Relation) -> Self {
        RelationDTO {
            id: value.id,

            organization_id: value.organization_id,
            branch_id: value.branch_id,
            user_id: value.user_id,
            role: value.role,
            relation_type: value.relation_type,

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct InviteToBranchDTO {
    #[validate(length(min = 1, max = 256, message = "User is not correct!"))]
    pub user_id: String,
    pub role: UserRoleType,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct RequestJoinToBranchDTO {
    #[validate(length(min = 1, max = 256, message = "Organization is not correct!"))]
    pub organization_id: String,
    #[validate(length(min = 1, max = 256, message = "Branch is not correct!"))]
    pub branch_id: String,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PatchInviteToBranchDTO {
    #[validate(length(min = 1, max = 256, message = "User is not correct!"))]
    pub user_id: Option<String>,
    pub role: Option<UserRoleType>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PatchRelationDTO {
    pub role: Option<UserRoleType>,
}
