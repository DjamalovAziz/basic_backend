use common::constant::DEFAULT_DOMAIN_PATH;
use common::enums::{RelationType, UserRoleType};
use common::functions::get_env_or;
use domain::models::management::{
    CreateRelation, CreateUser, PatchInviteToBranch, PatchRelation, PatchUser, PutUserPassword, Relation, User,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::sql::Thing;

// ==================== USER ==================== //

#[derive(Debug, Serialize)]
pub struct PutUserPasswordSurreal {
    pub password: String,

    pub updated_at: String,
}

impl From<PutUserPassword> for PutUserPasswordSurreal {
    fn from(value: PutUserPassword) -> Self {
        PutUserPasswordSurreal {
            password: value.password,
            updated_at: value.updated_at.to_string(),
        }
    }
}
//

#[derive(Debug, Deserialize)]
pub struct GetUserSurreal {
    pub id: Thing,

    pub password: String,
    pub image_path: String,
    pub phone_number: String,

    pub email: Option<String>,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetUserSurreal> for User {
    fn from(value: GetUserSurreal) -> Self {
        User {
            id: value.id.id.to_string(),

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

#[derive(Debug, Serialize)]
pub struct CreateUserSurreal {
    pub password: String,
    pub image_path: String,
    pub phone_number: String,

    pub email: Option<String>,

    pub created_at: String,
}

impl From<CreateUser> for CreateUserSurreal {
    fn from(value: CreateUser) -> Self {
        CreateUserSurreal {
            password: value.password,
            image_path: value.image_path,
            phone_number: value.phone_number,
            email: value.email,
            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PatchUserSurreal {
    pub image_path: Option<String>,
    pub phone_number: Option<String>,

    pub email: Option<String>,

    pub updated_at: String,
}

impl From<PatchUser> for PatchUserSurreal {
    fn from(value: PatchUser) -> Self {
        PatchUserSurreal {
            image_path: value.image_path,
            phone_number: value.phone_number,
            email: value.email,
            updated_at: value.updated_at.to_string(),
        }
    }
}

// ==================== Relation ==================== //

#[derive(Debug, Deserialize)]
pub struct GetRelationSurreal {
    pub id: Thing,

    pub organization_id: Thing,
    pub branch_id: Thing,
    pub user_id: Thing,
    pub role: UserRoleType,
    pub relation_type: RelationType,

    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<GetRelationSurreal> for Relation {
    fn from(value: GetRelationSurreal) -> Self {
        Relation {
            id: value.id.id.to_string(),

            branch_id: value.branch_id.id.to_string(),
            organization_id: value.organization_id.id.to_string(),
            user_id: value.user_id.id.to_string(),
            role: value.role,
            relation_type: value.relation_type,

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateRelationSurreal {
    pub organization_id: Thing,
    pub branch_id: Thing,
    pub user_id: Thing,
    pub role: UserRoleType,
    pub relation_type: RelationType,

    pub created_at: String,
}

impl From<CreateRelation> for CreateRelationSurreal {
    fn from(value: CreateRelation) -> Self {
        CreateRelationSurreal {
            branch_id: Thing::from(("branch", value.branch_id.as_str())),
            organization_id: Thing::from(("organization", value.organization_id.as_str())),
            user_id: Thing::from(("user", value.user_id.as_str())),
            role: value.role,
            relation_type: value.relation_type,

            created_at: value.created_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PatchInviteToBranchSurreal {
    pub user_id: Option<String>,
    pub role: Option<UserRoleType>,

    pub updated_at: String,
}

impl From<PatchInviteToBranch> for PatchInviteToBranchSurreal {
    fn from(value: PatchInviteToBranch) -> Self {
        PatchInviteToBranchSurreal {
            user_id: value.user_id,
            role: value.role,
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PatchRelationSurreal {
    pub role: Option<UserRoleType>,

    pub updated_at: String,
}

impl From<PatchRelation> for PatchRelationSurreal {
    fn from(value: PatchRelation) -> Self {
        PatchRelationSurreal {
            role: value.role,
            updated_at: value.updated_at.to_string(),
        }
    }
}
