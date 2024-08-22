use common::constant::DEFAULT_DOMAIN_PATH;
use common::enums::{RelationType, UserRoleType};
use common::functions::get_env_or;
use domain::models::management::{
    CreateRelation, CreateUser, PatchInviteToBranch, PatchRelation, PatchUser, PutUserPassword, Relation,
    User,
};

use crate::schemas::management::{relations, users};

use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

// ==================== USER ==================== //

#[derive(Debug, Serialize)]
pub struct PutUserPasswordDiesel {
    pub password: String,

    pub updated_at: String,
}

impl From<PutUserPassword> for PutUserPasswordDiesel {
    fn from(value: PutUserPassword) -> Self {
        PutUserPasswordDiesel {
            password: value.password,

            updated_at: value.updated_at.to_string(),
        }
    }
}
//

#[derive(Debug, Queryable)]
pub struct GetUserDiesel {
    pub id: String,

    pub password: String,
    pub image_path: String,
    pub phone_number: String,

    pub email: Option<String>,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetUserDiesel> for User {
    fn from(value: GetUserDiesel) -> Self {
        User {
            id: value.id,

            password: value.password,
            image_url: format!(
                "{}{}",
                get_env_or("DOMAIN_PATH", DEFAULT_DOMAIN_PATH),
                value.image_path.replacen('.', "", 1)
            ),
            phone_number: value.phone_number,

            email: value.email,

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct CreateUserDiesel {
    pub id: String,

    pub password: String,
    pub image_path: String,
    pub phone_number: String,

    pub email: Option<String>,

    pub created_at: String,
}

impl From<CreateUser> for CreateUserDiesel {
    fn from(value: CreateUser) -> Self {
        CreateUserDiesel {
            id: Uuid::new_v4().to_string(),

            password: value.password,
            image_path: value.image_path,
            phone_number: value.phone_number,

            email: value.email,

            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, AsChangeset, Serialize)]
#[table_name = "users"]
pub struct PatchUserDiesel {
    pub image_path: Option<String>,
    pub phone_number: Option<String>,

    pub email: Option<String>,

    pub updated_at: String,
}

impl From<PatchUser> for PatchUserDiesel {
    fn from(value: PatchUser) -> Self {
        PatchUserDiesel {
            image_path: value.image_path,
            phone_number: value.phone_number,
            email: value.email,
            updated_at: value.updated_at.to_string(),
        }
    }
}

// ==================== Relation ==================== //

#[derive(Debug, Queryable)]
pub struct GetRelationDiesel {
    pub id: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,
    pub role: UserRoleType,
    pub relation_type: RelationType,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetRelationDiesel> for Relation {
    fn from(value: GetRelationDiesel) -> Self {
        Relation {
            id: value.id,

            branch_id: value.branch_id,
            organization_id: value.organization_id,
            user_id: value.user_id,
            role: value.role,
            relation_type: value.relation_type,

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "relations"]
pub struct CreateRelationDiesel {
    pub id: String,

    pub organization_id: String,
    pub branch_id: String,
    pub user_id: String,
    pub role: UserRoleType,
    pub relation_type: RelationType,

    pub created_at: String,
}

impl From<CreateRelation> for CreateRelationDiesel {
    fn from(value: CreateRelation) -> Self {
        CreateRelationDiesel {
            id: Uuid::new_v4().to_string(),
            organization_id: value.organization_id,
            branch_id: value.branch_id,
            user_id: value.user_id,
            role: value.role,
            relation_type: value.relation_type,

            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, AsChangeset, Serialize)]
#[table_name = "relations"]
pub struct PatchInviteToBranchDiesel {
    pub user_id: Option<String>,
    pub role: Option<UserRoleType>,

    pub updated_at: String,
}

impl From<PatchInviteToBranch> for PatchInviteToBranchDiesel {
    fn from(value: PatchInviteToBranch) -> Self {
        PatchInviteToBranchDiesel {
            user_id: value.user_id,
            role: value.role,
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PatchRelationDiesel {
    pub role: Option<UserRoleType>,

    pub updated_at: String,
}

impl From<PatchRelation> for PatchRelationDiesel {
    fn from(value: PatchRelation) -> Self {
        PatchRelationDiesel {
            role: value.role,
            updated_at: value.updated_at.to_string(),
        }
    }
}
