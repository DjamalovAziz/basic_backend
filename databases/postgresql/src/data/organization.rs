use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use domain::models::organization::ForCall;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::schemas::organization::{branchs, organizations};

use domain::models::organization::{
    Branch, CreateBranch, CreateOrganization, Organization, PatchBranch, PatchOrganization,
};

// ==================== ORGANIZATION ==================== //
#[derive(Debug, Queryable)]
pub struct GetOrganizationDiesel {
    pub id: String,

    pub name: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetOrganizationDiesel> for Organization {
    fn from(value: GetOrganizationDiesel) -> Self {
        Organization {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "organizations"]
pub struct CreateOrganizationDiesel {
    pub id: String,

    pub name: String,

    pub created_at: String,
}

impl From<CreateOrganization> for CreateOrganizationDiesel {
    fn from(value: CreateOrganization) -> Self {
        CreateOrganizationDiesel {
            id: Uuid::new_v4().to_string(),
            name: value.name,
            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, AsChangeset, Serialize)]
#[table_name = "organizations"]
pub struct PatchOrganizationDiesel {
    pub name: Option<String>,

    pub updated_at: String,
}

impl From<PatchOrganization> for PatchOrganizationDiesel {
    fn from(value: PatchOrganization) -> Self {
        PatchOrganizationDiesel {
            name: value.name,
            updated_at: value.updated_at.to_string(),
        }
    }
}

// ==================== BRANCH ==================== //

#[derive(Debug, Queryable)]
pub struct GetBranchDiesel {
    pub id: String,

    pub name: String,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub organization_id: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetBranchDiesel> for Branch {
    fn from(value: GetBranchDiesel) -> Self {
        Branch {
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

#[derive(Debug, Insertable)]
#[table_name = "branchs"]
pub struct CreateBranchDiesel {
    pub id: String,

    pub name: String,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub organization_id: String,

    pub created_at: String,
}

impl From<CreateBranch> for CreateBranchDiesel {
    fn from(value: CreateBranch) -> Self {
        CreateBranchDiesel {
            id: Uuid::new_v4().to_string(),

            name: value.name,

            branch_location: value.branch_location,
            for_call: value.for_call,

            organization_id: value.organization_id,

            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, AsChangeset, Serialize)]
#[table_name = "branchs"]
pub struct PatchBranchDiesel {
    pub name: Option<String>,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub updated_at: String,
}

impl From<PatchBranch> for PatchBranchDiesel {
    fn from(value: PatchBranch) -> Self {
        PatchBranchDiesel {
            name: value.name,
            branch_location: value.branch_location,
            for_call: value.for_call,
            updated_at: value.updated_at.to_string(),
        }
    }
}
