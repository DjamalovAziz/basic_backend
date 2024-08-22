use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator_derive::Validate;

use crate::models::organization::{Branch, ForCall, Organization};

// ==================== ORGANIZATION ==================== //
#[derive(Debug, Serialize, ToSchema, IntoParams)]
pub struct OrganizationDTO {
    pub id: String,

    pub name: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<Organization> for OrganizationDTO {
    fn from(value: Organization) -> Self {
        OrganizationDTO {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PostOrganizationDTO {
    #[validate(length(min = 1, max = 256, message = "Organization name can't be empty!"))]
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PatchOrganizationDTO {
    #[validate(length(min = 1, max = 256, message = "Organization name can't be empty!"))]
    pub name: Option<String>,
}

// ==================== BRANCH ==================== //

#[derive(Debug, Serialize, ToSchema, IntoParams)]
pub struct BranchDTO {
    pub id: String,

    pub name: String,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub organization_id: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<Branch> for BranchDTO {
    fn from(value: Branch) -> Self {
        BranchDTO {
            id: value.id,
            name: value.name,

            branch_location: value.branch_location,
            for_call: value.for_call,

            organization_id: value.organization_id,

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PostBranchDTO {
    #[validate(length(min = 1, max = 256, message = "Branch name can't be empty!"))]
    pub name: String,

    #[validate(length(min = 1, max = 256, message = "Branch location can't be empty!"))]
    pub branch_location: Option<String>,
    #[validate(length(min = 1, max = 256, message = "Call data can't be empty!"))]
    pub for_call: Option<Vec<ForCall>>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PatchBranchDTO {
    #[validate(length(min = 1, max = 256, message = "Branch name can't be empty!"))]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 256, message = "Branch location can't be empty!"))]
    pub branch_location: Option<String>,
    #[validate(length(min = 1, max = 256, message = "Call data can't be empty!"))]
    pub for_call: Option<Vec<ForCall>>,

    //external fields:
    #[validate(length(min = 1, max = 256, message = "User is not correct!"))]
    pub user_id: Option<String>,
}
