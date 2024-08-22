use domain::models::organization::ForCall;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::sql::Thing;

use domain::models::organization::{
    Branch, CreateBranch, CreateOrganization, Organization, PatchBranch, PatchOrganization,
};

// ==================== ORGANIZATION ==================== //
#[derive(Debug, Deserialize)]
pub struct GetOrganizationSurreal {
    pub id: Thing,

    pub name: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetOrganizationSurreal> for Organization {
    fn from(value: GetOrganizationSurreal) -> Self {
        Organization {
            id: value.id.id.to_string(),
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateOrganizationSurreal {
    pub name: String,

    pub created_at: String,
}

impl From<CreateOrganization> for CreateOrganizationSurreal {
    fn from(value: CreateOrganization) -> Self {
        CreateOrganizationSurreal {
            name: value.name,
            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PatchOrganizationSurreal {
    pub name: Option<String>,

    pub updated_at: String,
}

impl From<PatchOrganization> for PatchOrganizationSurreal {
    fn from(value: PatchOrganization) -> Self {
        PatchOrganizationSurreal {
            name: value.name,
            updated_at: value.updated_at.to_string(),
        }
    }
}

// ==================== BRANCH ==================== //

#[derive(Debug, Deserialize)]
pub struct GetBranchSurreal {
    pub id: Thing,

    pub name: String,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub organization_id: Thing,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetBranchSurreal> for Branch {
    fn from(value: GetBranchSurreal) -> Self {
        Branch {
            id: value.id.id.to_string(),

            name: value.name,

            branch_location: value.branch_location,
            for_call: value.for_call,

            organization_id: value.organization_id.id.to_string(),

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateBranchSurreal {
    pub name: String,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub organization_id: Thing,

    pub created_at: String,
}

impl From<CreateBranch> for CreateBranchSurreal {
    fn from(value: CreateBranch) -> Self {
        CreateBranchSurreal {
            name: value.name,

            branch_location: value.branch_location,
            for_call: value.for_call,

            organization_id: Thing::from(("organization", value.organization_id.as_str())),

            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PatchBranchSurreal {
    pub name: Option<String>,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub updated_at: String,
}

impl From<PatchBranch> for PatchBranchSurreal {
    fn from(value: PatchBranch) -> Self {
        PatchBranchSurreal {
            name: value.name,
            branch_location: value.branch_location,
            for_call: value.for_call,
            updated_at: value.updated_at.to_string(),
        }
    }
}
