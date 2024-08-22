use common::enums::AdminRoleType;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::sql::Thing;

use domain::models::admin::{Admin, CreateAdmin, PatchAdmin, PutAdminPassword};

// ==================== ADMIN ==================== //
#[derive(Debug, Serialize)]
pub struct PutAdminPasswordSurreal {
    pub password: String,

    pub updated_at: String,
}

impl From<PutAdminPassword> for PutAdminPasswordSurreal {
    fn from(value: PutAdminPassword) -> Self {
        PutAdminPasswordSurreal {
            password: value.password,
            updated_at: value.updated_at.to_string(),
        }
    }
}
//

#[derive(Debug, Deserialize)]
pub struct GetAdminSurreal {
    pub id: Thing,

    pub password: String,
    pub role: AdminRoleType,
    pub phone_number: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetAdminSurreal> for Admin {
    fn from(value: GetAdminSurreal) -> Self {
        Admin {
            id: value.id.id.to_string(),
            password: value.password,
            role: value.role,
            phone_number: value.phone_number,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateAdminSurreal {
    pub password: String,
    pub role: AdminRoleType,
    pub phone_number: String,

    pub created_at: String,
}

impl From<CreateAdmin> for CreateAdminSurreal {
    fn from(value: CreateAdmin) -> Self {
        CreateAdminSurreal {
            password: value.password,
            phone_number: value.phone_number,
            role: value.role,
            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PatchAdminSurreal {
    pub role: Option<AdminRoleType>,
    pub phone_number: Option<String>,

    pub updated_at: String,
}

impl From<PatchAdmin> for PatchAdminSurreal {
    fn from(value: PatchAdmin) -> Self {
        PatchAdminSurreal {
            phone_number: value.phone_number,
            role: value.role,
            updated_at: value.updated_at.to_string(),
        }
    }
}
