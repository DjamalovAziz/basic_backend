use common::enums::UserRoleType::OrganizationOwner;
use common::errors::BasicError;
use common::types::BasicResult;
use domain::models::management::Relation;

// ==================== TelegramGroup Permission ==================== //
pub async fn telegram_group_create_ermission_controller(payload: Relation) -> BasicResult<(String, String)> {
    match payload.role {
        OrganizationOwner => Ok((payload.organization_id, payload.branch_id)),
        _ => Err(BasicError::forbidden_error(String::from("Forbidden!"))),
    }
}

pub async fn telegram_group_patch_ermission_controller(payload: Relation) -> BasicResult<()> {
    match payload.role {
        OrganizationOwner => Ok(()),
        _ => Err(BasicError::forbidden_error(String::from("Forbidden!"))),
    }
}

// ==================== FCMSubscription Permission ==================== //
pub async fn fcm_subscription_create_permission_controller(payload: Relation) -> BasicResult<(String, String, String)> {
    match payload.role {
        OrganizationOwner => Ok((payload.organization_id, payload.branch_id, payload.user_id)),
        _ => Err(BasicError::forbidden_error(String::from("Forbidden!"))),
    }
}

// ==================== Subscription Permission ==================== //
pub async fn subscription_create_permission_controller(payload: Relation) -> BasicResult<(String, String, String)> {
    match payload.role {
        OrganizationOwner => Ok((payload.organization_id, payload.branch_id, payload.user_id)),
        _ => Err(BasicError::forbidden_error(String::from("Forbidden!"))),
    }
}
