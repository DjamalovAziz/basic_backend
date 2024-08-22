use domain::repositories::organization::{BranchQueryParams, OrganizationQueryParams};
use domain::repositories::repository::{DEFAULT_LIMIT, DEFAULT_OFFSET, DEFAULT_START};

// ==================== ORGANIZATION ==================== //

pub trait OrganizationQueryParamsTrait: Send + Sync {
    fn query_params(&self) -> OrganizationQueryParams;
}

impl OrganizationQueryParamsTrait for OrganizationQueryParams {
    fn query_params(&self) -> OrganizationQueryParams {
        let mut conditions = Vec::new();

        if let Some(bot_token) = &self.bot_token {
            conditions.push(format!("bot_token = '{}'", bot_token));
        }
        if let (Some(created_at_from), Some(created_at_to)) = (&self.created_at_from, &self.created_at_to) {
            conditions.push(format!(
                "'{}' <= created_at || created_at <= '{}'",
                created_at_from, created_at_to
            ));
        }
        if let (Some(updated_at_from), Some(updated_at_to)) = (&self.updated_at_from, &self.updated_at_to) {
            conditions.push(format!(
                "'{}' <= updated_at || updated_at <= '{}'",
                updated_at_from, updated_at_to
            ));
        }

        let result = if !conditions.is_empty() {
            Some(format!("WHERE {}", conditions.join(" AND ")))
        } else {
            None
        };

        OrganizationQueryParams {
            limit: self.limit.or(Some(DEFAULT_LIMIT)),
            offset: self.offset.or(Some(DEFAULT_OFFSET)),
            start: self.start.or(Some(DEFAULT_START)),
            result,
            ..Default::default()
        }
    }
}

// ==================== BRANCH ==================== //

pub trait BranchQueryParamsTrait: Send + Sync {
    fn query_params(&self) -> BranchQueryParams;
}

impl BranchQueryParamsTrait for BranchQueryParams {
    fn query_params(&self) -> BranchQueryParams {
        let mut conditions = Vec::new();

        if let Some(organization_id) = &self.organization_id {
            conditions.push(format!("organization_id = organization:{}", organization_id));
        }
        if let Some(created_at) = &self.created_at {
            conditions.push(format!("created_at = '{}'", created_at));
        }
        if let Some(updated_at) = &self.updated_at {
            conditions.push(format!("updated_at = '{}'", updated_at));
        }

        let result = if !conditions.is_empty() {
            Some(format!("WHERE {}", conditions.join(" AND ")))
        } else {
            None
        };

        BranchQueryParams {
            limit: self.limit.or(Some(DEFAULT_LIMIT)),
            offset: self.offset.or(Some(DEFAULT_OFFSET)),
            start: self.start.or(Some(DEFAULT_START)),
            result,
            ..Default::default()
        }
    }
}
