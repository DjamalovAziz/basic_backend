use domain::repositories::admin::AdminQueryParams;
use domain::repositories::repository::{DEFAULT_LIMIT, DEFAULT_OFFSET, DEFAULT_START};

pub trait AdminQueryParamsTrait: Send + Sync {
    fn query_params(&self) -> AdminQueryParams;
}

impl AdminQueryParamsTrait for AdminQueryParams {
    fn query_params(&self) -> AdminQueryParams {
        let mut conditions = Vec::new();

        if let Some(role) = &self.role {
            conditions.push(format!("role = '{}'", role));
        }
        if let (Some(created_at_from), Some(created_at_to)) = (&self.created_at_from, &self.created_at_to) {
            conditions.push(format!("('{}' <= created_at <= '{}')", created_at_from, created_at_to));
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

        AdminQueryParams {
            limit: self.limit.or(Some(DEFAULT_LIMIT)),
            offset: self.offset.or(Some(DEFAULT_OFFSET)),
            start: self.start.or(Some(DEFAULT_START)),
            result,
            ..Default::default()
        }
    }
}
