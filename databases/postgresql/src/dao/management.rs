use domain::repositories::management::{RelationQueryParams, UserQueryParams};
use domain::repositories::repository::{DEFAULT_LIMIT, DEFAULT_OFFSET, DEFAULT_START};

// ==================== USER ==================== //

pub trait UserQueryParamsTrait: Send + Sync {
    fn query_params(&self) -> UserQueryParams;
}

impl UserQueryParamsTrait for UserQueryParams {
    fn query_params(&self) -> UserQueryParams {
        let mut conditions = Vec::new();

        if let (Some(created_at_from), Some(created_at_to)) = (&self.created_at_from, &self.created_at_to) {
            conditions.push(format!(
                "created_at BETWEEN '{}' AND '{}'",
                created_at_from, created_at_to
            ));
        }
        if let (Some(updated_at_from), Some(updated_at_to)) = (&self.updated_at_from, &self.updated_at_to) {
            conditions.push(format!(
                "updated_at BETWEEN '{}' AND '{}'",
                updated_at_from, updated_at_to
            ));
        }

        let where_clause = if !conditions.is_empty() {
            Some(format!("WHERE {}", conditions.join(" AND ")))
        } else {
            None
        };

        UserQueryParams {
            limit: self.limit.or(Some(DEFAULT_LIMIT)),
            offset: self.offset.or(Some(DEFAULT_OFFSET)),
            start: self.start.or(Some(DEFAULT_START)),
            result: where_clause,
            ..Default::default()
        }
    }
}

pub trait RelationQueryParamsTrait: Send + Sync {
    fn query_params(&self) -> RelationQueryParams;
}

impl RelationQueryParamsTrait for RelationQueryParams {
    fn query_params(&self) -> RelationQueryParams {
        let mut conditions = Vec::new();

        if let Some(organization_id) = &self.organization_id {
            conditions.push(format!("organization_id = '{}'", organization_id));
        }
        if let Some(branch_id) = &self.branch_id {
            conditions.push(format!("branch_id = '{}'", branch_id));
        }
        if let Some(role) = &self.role {
            conditions.push(format!("role = '{}'", role));
        }
        if let Some(relation_type) = &self.relation_type {
            conditions.push(format!("relation_type = '{}'", relation_type));
        }
        if let (Some(created_at_from), Some(created_at_to)) = (&self.created_at_from, &self.created_at_to) {
            conditions.push(format!(
                "created_at BETWEEN '{}' AND '{}'",
                created_at_from, created_at_to
            ));
        }
        if let (Some(updated_at_from), Some(updated_at_to)) = (&self.updated_at_from, &self.updated_at_to) {
            conditions.push(format!(
                "updated_at BETWEEN '{}' AND '{}'",
                updated_at_from, updated_at_to
            ));
        }

        let where_clause = if !conditions.is_empty() {
            Some(format!("WHERE {}", conditions.join(" AND ")))
        } else {
            None
        };

        RelationQueryParams {
            limit: self.limit.or(Some(DEFAULT_LIMIT)),
            offset: self.offset.or(Some(DEFAULT_OFFSET)),
            start: self.start.or(Some(DEFAULT_START)),
            result: where_clause,
            ..Default::default()
        }
    }
}
