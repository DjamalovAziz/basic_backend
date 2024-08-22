use common::enums::AdminRoleType::{Admin as RoleTypeAdmin, SuperAdmin};
use common::errors::BasicError;
use common::types::BasicResult;
use domain::models::admin::Admin;

// ==================== ADMIN Permission ==================== //

pub async fn admin_get_list_create_permission_controller(admin: Admin) -> BasicResult<()> {
    match admin.role {
        SuperAdmin => Ok(()),
        _ => Err(BasicError::forbidden_error(String::from("Forbidden!"))),
    }
}

pub async fn admin_permission_controller(id: &str, admin: Admin) -> BasicResult<()> {
    if admin.role == SuperAdmin || admin.role == RoleTypeAdmin && *id == admin.id {
        Ok(())
    } else {
        Err(BasicError::forbidden_error(String::from("Forbidden!")))
    }
}

pub async fn admin_get_me_admin_permission_controller(admin: Admin) -> BasicResult<()> {
    if admin.role == SuperAdmin || admin.role == RoleTypeAdmin {
        Ok(())
    } else {
        Err(BasicError::forbidden_error(String::from("Forbidden!")))
    }
}
