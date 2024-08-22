use common::enums::UserRoleType::OrganizationOwner;
use common::errors::BasicError;
use common::types::BasicResult;
use domain::models::management::Relation;

pub async fn relation_id_permission_controller(
    relations: Vec<Relation>,
    organization_id: &str,
    branch_id: &str,
    current_id: &str,
) -> BasicResult<()> {
    if relations.iter().any(|relation| {
        relation.role == OrganizationOwner
            && relation.organization_id == *organization_id
            && relation.branch_id == *branch_id
    }) {
        Ok(())
    } else if relations.iter().any(|relation| {
        relation.organization_id == *organization_id && relation.branch_id == *branch_id && relation.id == *current_id
    }) {
        Ok(())
    } else {
        Err(BasicError::forbidden_error(String::from("Forbidden!")))
    }
}
