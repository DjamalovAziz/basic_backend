use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::enums::ProductType;
use utoipa::ToSchema;

// ==================== ORGANIZATION ==================== //
#[derive(Serialize, Debug, Deserialize, Default)]
pub struct Organization {
    pub id: String,

    pub name: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateOrganization {
    pub name: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchOrganization {
    pub name: Option<String>,

    pub updated_at: DateTime<Utc>,
}

// ==================== BRANCH ==================== //
#[derive(Serialize, Debug, Deserialize, ToSchema)]
pub struct ForCall {
    pub name: String,
    pub phone_number: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Branch {
    pub id: String,

    pub name: String,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub organization_id: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateBranch {
    pub name: String,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub organization_id: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchBranch {
    pub name: Option<String>,

    pub branch_location: Option<String>,
    pub for_call: Option<Vec<ForCall>>,

    pub updated_at: DateTime<Utc>,
}

// ==================== Product ==================== //
#[derive(Serialize, Debug, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub r#type: ProductType,
    pub price: u32,
}
