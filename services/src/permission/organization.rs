use common::enums::UserRoleType::OrganizationOwner;
use common::errors::BasicError;
use common::types::BasicResult;
use domain::models::management::Relation;

// Organization //
pub async fn organization_delete_patch_permission_controller(
    relations: Vec<Relation>,
    current_id: &str,
) -> BasicResult<()> {
    if relations
        .iter()
        .any(|relation| relation.role == OrganizationOwner && relation.organization_id == *current_id)
    {
        Ok(())
    } else {
        Err(BasicError::forbidden_error(String::from("Forbidden!")))
    }
}

// Branch //
pub async fn branch_create_permission_controller(
    relations: Vec<Relation>,
    organization_id: &str,
) -> BasicResult<()> {
    if relations
        .iter()
        .any(|relation| relation.role == OrganizationOwner && relation.organization_id == *organization_id)
    {
        Ok(())
    } else {
        Err(BasicError::forbidden_error(String::from("Forbidden!")))
    }
}

pub async fn branch_delete_permission_controller(
    relations: Vec<Relation>,
    organization_id: &str,
    current_id: &str,
) -> BasicResult<()> {
    if relations.iter().any(|relation| {
        relation.role == OrganizationOwner
            && relation.organization_id == *organization_id
            && relation.branch_id == *current_id
    }) {
        Ok(())
    } else {
        Err(BasicError::forbidden_error(String::from("Forbidden!")))
    }
}

pub async fn branch_patch_permission_controller(
    relations: Vec<Relation>,
    organization_id: &str,
    current_id: &str,
) -> BasicResult<()> {
    if relations.iter().any(|relation| {
        relation.role == OrganizationOwner
            && relation.organization_id == *organization_id
            && relation.branch_id == *current_id
    }) {
        Ok(())
    } else {
        Err(BasicError::forbidden_error(String::from("Forbidden!")))
    }
}
