use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::sql_types::VarChar;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use utoipa::ToSchema;

// ==================== ADMIN ==================== //
#[derive(Debug, Default, Deserialize, PartialEq, Serialize, ToSchema, AsExpression, FromSqlRow)]
#[sql_type = "VarChar"]
pub enum AdminRoleType {
    #[default]
    Admin,
    SuperAdmin,
}

impl Display for AdminRoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdminRoleType::Admin => {
                write!(f, "Admin")
            }
            AdminRoleType::SuperAdmin => {
                write!(f, "SuperAdmin")
            }
        }
    }
}

impl From<String> for AdminRoleType {
    fn from(value: String) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "admin" => AdminRoleType::Admin,
            "superadmin" => AdminRoleType::SuperAdmin,
            _ => AdminRoleType::Admin,
        }
    }
}

impl<POSTGRESQL> diesel::deserialize::FromSql<VarChar, POSTGRESQL> for AdminRoleType
where
    POSTGRESQL: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<VarChar, POSTGRESQL>,
{
    fn from_sql(bytes: <POSTGRESQL as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let admin_role_type = String::from_sql(bytes)?;
        match admin_role_type.as_str() {
            "Admin" => Ok(AdminRoleType::Admin),
            "SuperAdmin" => Ok(AdminRoleType::SuperAdmin),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl<POSTGRESQL> diesel::serialize::ToSql<VarChar, POSTGRESQL> for AdminRoleType
where
    POSTGRESQL: diesel::backend::Backend,
    str: diesel::serialize::ToSql<VarChar, POSTGRESQL>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, POSTGRESQL>) -> diesel::serialize::Result {
        let admin_role_type_str = match *self {
            AdminRoleType::Admin => "Admin",
            AdminRoleType::SuperAdmin => "SuperAdmin",
        };
        admin_role_type_str.to_sql(out)
    }
}

// ==================== MANAGEMENT ==================== //
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, ToSchema, AsExpression, FromSqlRow)]
#[sql_type = "VarChar"]
pub enum UserRoleType {
    #[default]
    Member,
    OrganizationOwner,
}

impl<POSTGRESQL> diesel::deserialize::FromSql<VarChar, POSTGRESQL> for UserRoleType
where
    POSTGRESQL: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<VarChar, POSTGRESQL>,
{
    fn from_sql(bytes: <POSTGRESQL as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let user_role_type = String::from_sql(bytes)?;
        match user_role_type.as_str() {
            "Member" => Ok(UserRoleType::Member),
            "OrganizationOwner" => Ok(UserRoleType::OrganizationOwner),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl<POSTGRESQL> diesel::serialize::ToSql<VarChar, POSTGRESQL> for UserRoleType
where
    POSTGRESQL: diesel::backend::Backend,
    str: diesel::serialize::ToSql<VarChar, POSTGRESQL>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, POSTGRESQL>) -> diesel::serialize::Result {
        let user_role_type_str = match *self {
            UserRoleType::Member => "Member",
            UserRoleType::OrganizationOwner => "OrganizationOwner",
        };
        user_role_type_str.to_sql(out)
    }
}

impl Display for UserRoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRoleType::Member => {
                write!(f, "Member")
            }
            UserRoleType::OrganizationOwner => {
                write!(f, "OrganizationOwner")
            }
        }
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize, ToSchema, AsExpression, FromSqlRow)]
#[sql_type = "VarChar"]
pub enum RelationType {
    #[default]
    Relation,
    RequestToJoin,
    InvitationToUser,
}

impl<POSTGRESQL> diesel::deserialize::FromSql<VarChar, POSTGRESQL> for RelationType
where
    POSTGRESQL: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<VarChar, POSTGRESQL>,
{
    fn from_sql(bytes: <POSTGRESQL as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let relation_type: String = String::from_sql(bytes)?;
        match relation_type.as_str() {
            "Relation" => Ok(RelationType::Relation),
            "RequestToJoin" => Ok(RelationType::RequestToJoin),
            "InvitationToUser" => Ok(RelationType::InvitationToUser),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl<POSTGRESQL> diesel::serialize::ToSql<VarChar, POSTGRESQL> for RelationType
where
    POSTGRESQL: diesel::backend::Backend,
    str: diesel::serialize::ToSql<VarChar, POSTGRESQL>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, POSTGRESQL>) -> diesel::serialize::Result {
        let relation_type_str = match *self {
            RelationType::Relation => "Relation",
            RelationType::RequestToJoin => "RequestToJoin",
            RelationType::InvitationToUser => "InvitationToUser",
        };
        relation_type_str.to_sql(out)
    }
}

impl Display for RelationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationType::Relation => {
                write!(f, "Relation")
            }
            RelationType::RequestToJoin => {
                write!(f, "RequestToJoin")
            }
            RelationType::InvitationToUser => {
                write!(f, "InvitationToUser")
            }
        }
    }
}

// ==================== ORGANIZATION ==================== //
#[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
pub enum ProductType {
    #[default]
    Bronze,
    Silver,
    Golden,
    Platinum,
}
