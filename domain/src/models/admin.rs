use chrono::{DateTime, SubsecRound, Utc};

use common::enums::AdminRoleType;
use common::functions::generate_hash;
use serde::{Deserialize, Serialize};

use crate::dto::admin::{CreateAdminCLIDTO, PatchAdminCLIDTO, SignUpAdminCLIDTO};

// ==================== ADMIN ==================== //

#[derive(Debug, Serialize, Deserialize)]
pub struct PutAdminPassword {
    pub password: String,

    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Admin {
    pub id: String,

    pub password: String,
    pub role: AdminRoleType,
    pub phone_number: String,

    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAdmin {
    pub password: String,
    pub role: AdminRoleType,
    pub phone_number: String,

    pub created_at: DateTime<Utc>,
}

impl From<CreateAdminCLIDTO> for CreateAdmin {
    fn from(value: CreateAdminCLIDTO) -> Self {
        CreateAdmin {
            password: generate_hash(value.password.as_bytes()).unwrap_or_default(),
            role: value.role,
            phone_number: value.phone_number,
            created_at: Utc::now().trunc_subsecs(0),
        }
    }
}

impl From<SignUpAdminCLIDTO> for CreateAdmin {
    fn from(value: SignUpAdminCLIDTO) -> Self {
        CreateAdmin {
            password: generate_hash(value.password.as_bytes()).unwrap_or_default(),
            role: AdminRoleType::SuperAdmin,
            phone_number: value.phone_number,
            created_at: Utc::now().trunc_subsecs(0),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchAdmin {
    pub phone_number: Option<String>,
    pub role: Option<AdminRoleType>,

    pub updated_at: DateTime<Utc>,
}

impl From<PatchAdminCLIDTO> for PatchAdmin {
    fn from(value: PatchAdminCLIDTO) -> Self {
        PatchAdmin {
            role: value.role,
            phone_number: value.phone_number,
            updated_at: Utc::now().trunc_subsecs(0),
        }
    }
}
