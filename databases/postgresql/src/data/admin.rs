use diesel::deserialize::FromSqlRow;
use diesel::sql_types::{BigInt, VarChar};
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName};
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::schemas::admin::admins;

use common::enums::AdminRoleType;
use domain::models::admin::PutAdminPassword;
use domain::models::admin::{Admin, CreateAdmin, PatchAdmin};

// ==================== ADMIN ==================== //

#[derive(Debug, AsChangeset, Serialize)]
#[table_name = "admins"]
pub struct PutAdminPasswordDiesel {
    pub password: String,

    pub updated_at: String,
}

impl From<PutAdminPassword> for PutAdminPasswordDiesel {
    fn from(value: PutAdminPassword) -> Self {
        PutAdminPasswordDiesel {
            password: value.password,

            updated_at: value.updated_at.to_string(),
        }
    }
}
//
#[derive(Debug, Queryable)]
pub struct GetAdminDiesel {
    pub id: String,

    pub password: String,
    pub role: AdminRoleType,
    pub phone_number: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetAdminDiesel> for Admin {
    fn from(value: GetAdminDiesel) -> Self {
        Admin {
            id: value.id,
            password: value.password,
            role: value.role,
            phone_number: value.phone_number,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(QueryableByName, Queryable)]
pub struct AdminWithTotal {
    #[sql_type = "VarChar"]
    pub id: String,

    #[sql_type = "VarChar"]
    pub password: String,

    #[sql_type = "VarChar"]
    pub role: AdminRoleType,

    #[sql_type = "VarChar"]
    pub phone_number: String,

    #[sql_type = "VarChar"]
    pub created_at: String,

    #[sql_type = "VarChar"]
    pub updated_at: Option<String>,

    #[sql_type = "BigInt"]
    pub total: u64,
}

impl From<AdminWithTotal> for GetAdminDiesel {
    fn from(value: AdminWithTotal) -> Self {
        GetAdminDiesel {
            id: value.id,
            password: value.password,
            role: value.role,
            phone_number: value.phone_number,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "admins"]
pub struct CreateAdminDiesel {
    pub id: String,

    pub password: String,
    pub role: AdminRoleType,
    pub phone_number: String,

    pub created_at: String,
}

impl From<CreateAdmin> for CreateAdminDiesel {
    fn from(value: CreateAdmin) -> Self {
        CreateAdminDiesel {
            id: Uuid::new_v4().to_string(),
            password: value.password,
            phone_number: value.phone_number,
            role: value.role,
            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, AsChangeset, Serialize)]
#[table_name = "admins"]
pub struct PatchAdminDiesel {
    pub role: Option<AdminRoleType>,
    pub phone_number: Option<String>,

    pub updated_at: String,
}

impl From<PatchAdmin> for PatchAdminDiesel {
    fn from(value: PatchAdmin) -> Self {
        PatchAdminDiesel {
            phone_number: value.phone_number,
            role: value.role,
            updated_at: value.updated_at.to_string(),
        }
    }
}
