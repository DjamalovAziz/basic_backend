use std::sync::Arc;

use async_trait::async_trait;
use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::{prelude::*, sql_query};

use common::errors::BasicError;
use common::responses::DeleteResponseResult;
use common::types::BasicResult;
use domain::models::admin::{Admin, CreateAdmin, PatchAdmin, PutAdminPassword};
use domain::repositories::admin::{AdminQueryParams, AdminTrait};
use domain::repositories::repository::{ResultPaging, ResultPagingDB, DEFAULT_LIMIT, DEFAULT_OFFSET};

use crate::dao::admin::AdminQueryParamsTrait;
use crate::data::admin::{
    AdminWithTotal, CreateAdminDiesel, GetAdminDiesel, PatchAdminDiesel, PutAdminPasswordDiesel,
};

pub struct AdminDieselRepository {
    pub pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl AdminDieselRepository {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        AdminDieselRepository { pool }
    }

    fn get_conn(&self) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, BasicError> {
        self.pool
            .get()
            .map_err(|e| BasicError::server_error(format!("Failed to get database connection: {}", e)))
    }
}

#[async_trait]
impl AdminTrait for AdminDieselRepository {
    async fn change_password(&self, id: String, data: PutAdminPassword) -> BasicResult<String> {
        use crate::schemas::admin::admins::dsl::*;

        let result = diesel::update(admins.filter(id.eq(&id)))
            .set(&PutAdminPasswordDiesel::from(data))
            .execute(&mut self.get_conn()?)
            .map_err(|e| BasicError::server_error(format!("Failed to update password: {}", e)))?;

        if result == 0 {
            Err(BasicError::not_found_error(String::from("Admin not found!")))
        } else {
            Ok(String::from("Password successfully updated!"))
        }
    }

    async fn get_by_phone_number(&self, phone_number: &str) -> BasicResult<Admin> {
        use crate::schemas::admin::admins::dsl::*;

        let admin: GetAdminDiesel = admins
            .filter(phone_number.eq(phone_number))
            .get_result::<GetAdminDiesel>(&mut self.get_conn()?)
            .map_err(|e| {
                BasicError::not_found_error(format!("Failed to find admin by phone number: {}", e))
            })?;

        Ok(admin.into())
    }

    async fn is_superadmin_in_db(&self) -> BasicResult<bool> {
        use crate::schemas::admin::admins::dsl::*;

        let result = admins
            .filter(role.eq("SuperAdmin"))
            .count()
            .get_result::<i64>(&mut self.get_conn()?)
            .map_err(|e| BasicError::server_error(format!("Failed to check for superadmin: {}", e)))?;

        Ok(result > 0)
    }

    async fn merge_cli(&self, current_phone_number: String, admin: PatchAdmin) -> BasicResult<Admin> {
        use crate::schemas::admin::admins::dsl::*;
        todo!()
    }

    async fn create(&self, admin: CreateAdmin) -> BasicResult<Admin> {
        use crate::schemas::admin::admins::dsl::*;

        let admin = diesel::insert_into(admins)
            .values(&CreateAdminDiesel::from(admin))
            .get_result::<GetAdminDiesel>(&mut self.get_conn()?)
            .map_err(|e| BasicError::cannot_create_error(format!("Failed to create admin: {}", e)))?;

        Ok(admin.into())
    }

    async fn delete(&self, id: String) -> BasicResult<DeleteResponseResult> {
        use crate::schemas::admin::admins::dsl::*;

        let result = diesel::delete(admins.filter(id.eq(id)))
            .execute(&mut self.get_conn()?)
            .map_err(|e| BasicError::server_error(format!("Failed to delete admin: {}", e)))?;

        if result > 0 {
            Ok(DeleteResponseResult { status_code: 204 })
        } else {
            Err(BasicError::not_found_error(String::from("Admin not found!")))
        }
    }

    async fn get(&self, id: &str) -> BasicResult<Admin> {
        use crate::schemas::admin::admins::dsl::*;

        let admin = admins
            .find(id)
            .get_result::<GetAdminDiesel>(&mut self.get_conn()?)
            .map_err(|e| BasicError::not_found_error(format!("Admin not found: {}", e)))?;

        Ok(admin.into())
    }

    async fn list(&self, query_params: AdminQueryParams) -> BasicResult<ResultPaging<Admin>> {
        use crate::schemas::admin::admins::dsl::*;

        let query_data = query_params.query_params();

        let conn = &mut self.get_conn()?;

        let results = admins
            .select((
                id,
                password,
                role,
                phone_number,
                created_at,
                updated_at,
                diesel::dsl::sql::<diesel::sql_types::BigInt>("(SELECT COUNT(*) FROM admins) AS total"),
            ))
            .filter(match query_data.result {
                Some(value) => phone_number.like(value),
                None => diesel::dsl::true_dsl(), // No filter if None
            })
            .offset(query_data.offset.unwrap_or(DEFAULT_OFFSET))
            .limit(query_data.limit.unwrap_or(DEFAULT_LIMIT))
            .load::<AdminWithTotal>(conn)
            .map_err(|e| BasicError::server_error(format!("Failed to load admins: {}", e)))?;

        if results.is_empty() {
            return Err(BasicError::not_found_error(String::from("No admins found!")));
        }

        let total = results.first().map(|res| res.total as usize).unwrap_or(0);
        let admins: Vec<GetAdminDiesel> = results.into_iter().map(Into::into).collect();

        Ok(ResultPaging::new(
            total,
            query_data.limit.unwrap_or(DEFAULT_LIMIT) as usize,
            query_data.offset.unwrap_or(DEFAULT_OFFSET) as usize,
            admins.into_iter().map(Into::into).collect(),
        ))
    }

    async fn patch(&self, id: String, data: PatchAdmin) -> BasicResult<Admin> {
        use crate::schemas::admin::admins::dsl::*;

        let affected_rows = diesel::update(admins.filter(id.eq(&id)))
            .set(&PatchAdminDiesel::from(data))
            .execute(&mut self.get_conn()?)
            .map_err(|e| BasicError::server_error(format!("Failed to update password: {}", e)))?;

        if affected_rows == 0 {
            return Err(BasicError::not_found_error(String::from("Admin not found!")));
        }

        let result: GetAdminDiesel = admins
            .find(id)
            .first(&mut self.get_conn()?)
            .map_err(|e| BasicError::server_error(format!("Failed to fetch updated admin: {}", e)))?;

        Ok(result.into())
    }
}
