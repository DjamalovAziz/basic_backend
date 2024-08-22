use std::sync::Arc;

use async_trait::async_trait;
use common::responses::DeleteResponseResult;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::dao::admin::AdminQueryParamsTrait;
use crate::data::admin::{CreateAdminSurreal, GetAdminSurreal, PatchAdminSurreal, PutAdminPasswordSurreal};

use common::errors::BasicError;
use common::types::BasicResult;
use domain::models::admin::Admin;
use domain::models::admin::{CreateAdmin, PatchAdmin, PutAdminPassword};
use domain::repositories::admin::{AdminQueryParams, AdminTrait};
use domain::repositories::repository::{ResultPaging, ResultPagingDB};

// ==================== ADMIN ==================== //
pub struct AdminSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl AdminSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        AdminSurrealRepository { pool: db }
    }
}

#[async_trait]
impl AdminTrait for AdminSurrealRepository {
    async fn change_password(&self, id: String, data: PutAdminPassword) -> BasicResult<String> {
        let result: Option<GetAdminSurreal> = self
            .pool
            .update(("admin", id))
            .merge(PutAdminPasswordSurreal::from(data))
            .await?;

        match result {
            Some(_) => Ok(String::from("Password successfully updated!")),
            None => Err(BasicError::not_found_error(String::from("Admin not found!"))),
        }
    }

    async fn get_by_phone_number(&self, phone_number: &str) -> BasicResult<Admin> {
        let result: Option<GetAdminSurreal> = self
            .pool
            .query("SELECT * FROM admin WHERE phone_number = $phone_number;")
            .bind(("phone_number", &phone_number))
            .await?
            .take(0)?;
        match result {
            Some(admin) => Ok(admin.into()),
            None => Err(BasicError::not_found_error(String::from(
                "Admin by current phone number not found!",
            ))),
        }
    }

    async fn is_superadmin_in_db(&self) -> BasicResult<bool> {
        let result: Option<GetAdminSurreal> = self
            .pool
            .query("SELECT * FROM admin WHERE role = 'SuperAdmin';")
            .await?
            .take(0)?;

        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn merge_cli(&self, current_phone_number: String, admin: PatchAdmin) -> BasicResult<Admin> {
        let mut string = String::from("");

        if let Some(ref role) = admin.role {
            string = format!("{} role = '{:?}' ", string, role);
        } else if let Some(phone_number) = admin.phone_number.clone() {
            string = format!("{} phone_number = '{}' ", string, phone_number);
        }
        let result: Option<GetAdminSurreal> = self
            .pool
            .query(format!(
                "UPDATE admin SET {} WHERE phone_number = $current_phone_number;",
                string
            ))
            .bind(("current_phone_number", current_phone_number))
            .await?
            .take(0)?;

        match result {
            Some(admin) => Ok(admin.into()),
            None => Err(BasicError::not_found_error(String::from("Admin not found!"))),
        }
    }

    //
    async fn create(&self, admin: CreateAdmin) -> BasicResult<Admin> {
        let result: Option<GetAdminSurreal> = self
            .pool
            .create("admin")
            .content(CreateAdminSurreal::from(admin))
            .await?
            .pop();

        match result {
            Some(admin_surreal) => Ok(admin_surreal.into()),
            None => Err(BasicError::cannot_create_error(String::from(
                "Admin cannot be created!",
            ))),
        }
    }

    async fn delete(&self, id: String) -> BasicResult<DeleteResponseResult> {
        let result_delete: Option<GetAdminSurreal> = self.pool.delete(("admin", id)).await?;
        match result_delete.is_some() {
            true => Ok(DeleteResponseResult { status_code: 204 }),
            false => Err(BasicError::not_found_error(String::from("Admin not found!"))),
        }
    }

    async fn get(&self, id: &str) -> BasicResult<Admin> {
        let result: Option<GetAdminSurreal> = self.pool.select(("admin", id)).await?;

        match result {
            Some(admin) => Ok(admin.into()),
            None => Err(BasicError::not_found_error(String::from("Admin not found!"))),
        }
    }

    async fn list(&self, query_params: AdminQueryParams) -> BasicResult<ResultPaging<Admin>> {
        let query_data = query_params.query_params();

        let result: Option<ResultPagingDB<GetAdminSurreal>> = self
            .pool
            .query(format!(
                "$limit = {}; $offset = {}; $start = {}; $result = SELECT * FROM admin {} START $start LIMIT $limit;\
                RETURN {{total: COUNT(SELECT count() FROM admin), limit: $limit, count: COUNT($result), page: $offset, items: $result}};",
                query_data.limit.unwrap_or_default(),
                query_data.offset.unwrap_or_default(),
                query_data.start.unwrap_or_default(),
                query_data.result.unwrap_or_default(),
            ))
            .await?
            .take(4)?;

        result
            .ok_or(BasicError::bad_request_error(String::from("Bad request!")))
            .map(ResultPagingDB::into)
    }

    async fn patch(&self, id: String, admin: PatchAdmin) -> BasicResult<Admin> {
        let result: Option<GetAdminSurreal> = self
            .pool
            .update(("admin", id))
            .merge(PatchAdminSurreal::from(admin))
            .await?;
        match result {
            Some(admin) => Ok(admin.into()),
            None => Err(BasicError::not_found_error(String::from("Admin not found!"))),
        }
    }
}
