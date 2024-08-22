use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use std::sync::Arc;

use domain::dto::admin::{AdminDTO, PatchAdminDTO, PostAdminDTO, PutAdminPasswordDTO, SignInAdminDTO};
use domain::dto::management::PhoneNumberDTO;
use domain::repositories::admin::AdminQueryParams;
use domain::repositories::repository::{ResultPaging, Token};
use domain::services::admin::AdminService;
use common::errors::BasicError;

use crate::middleware::AuthorizationService;

// ==================== ADMIN ==================== //
#[utoipa::path(
    tag = "admin",
    security(("token" = [])),
)]
pub async fn change_admin_password_handler(
    auth: AuthorizationService,
    service: &dyn AdminService,
    Path(id): Path<String>,
    Json(data): Json<PutAdminPasswordDTO>,
) -> Result<Json<String>, StatusCode> {
    service
        .change_password(id, data, auth.id)
        .await
        .map_err(BasicError::from)
        .map(|_| Json("Password changed successfully".to_string()))
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

#[utoipa::path(
    tag = "admin",
    security(("token" = [])),
)]
#[get("/admins/me")]
pub async fn get_me_admin_handler(
    auth: AuthorizationService,
    State(service): State<dyn AdminService>,
) -> Result<Json<AdminDTO>, StatusCode> {
    service
        .get_me_admin(auth.id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

#[utoipa::path(tag = "admin")]
#[put("/admins/reset_admin_password")]
pub async fn reset_admin_password_handler(
    State(service): State<dyn AdminService>,
    Json(data): Json<PhoneNumberDTO>,
) -> Result<Json<String>, StatusCode> {
    service
        .reset_admin_password(&data.phone_number)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

#[utoipa::path(tag = "admin")]
#[post("/admins/signin")]
pub async fn signin_admin_handler(
    State(service): State<dyn AdminService>,
    Json(data): Json<SignInAdminDTO>,
) -> Result<Json<Token>, StatusCode> {
    service
        .signin_admin(data)
        .await
        .map(Json)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

// BASIC
#[utoipa::path(
    tag = "admin",
    security(("token" = [])),
    params(AdminQueryParams),
)]
#[get("/admins")]
pub async fn list_admins_handler(
    auth: AuthorizationService,
    State(service): State<dyn AdminService>,
    Query(params): Query<AdminQueryParams>,
) -> Result<Json<ResultPaging<AdminDTO>>, StatusCode> {
    service
        .list(params, auth.id)
        .await
        .map(|result| Json(result.paging_from()))
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

#[utoipa::path(
    tag = "admin",
    security(("token" = [])),
)]
#[get("/admins/:id")]
pub async fn get_admin_handler(
    auth: AuthorizationService,
    State(service): State<dyn AdminService>,
    Path(id): Path<String>,
) -> Result<Json<AdminDTO>, StatusCode> {
    service.get(id, auth.id).await.map(Json).map_err(|err| match err {
        BasicError::NotFound(_) => StatusCode::NOT_FOUND,
        _ => StatusCode::UNAUTHORIZED,
    })
}

#[utoipa::path(
    tag = "admin",
    security(("token" = [])),
)]
#[post("/admins")]
pub async fn post_admin_handler(
    auth: AuthorizationService,
    State(service): State<dyn AdminService>,
    Json(data): Json<PostAdminDTO>,
) -> Result<Json<AdminDTO>, StatusCode> {
    service.create(data, auth.id).await.map(Json).map_err(|err| match err {
        BasicError::ValidationError(_) => StatusCode::BAD_REQUEST,
        _ => StatusCode::UNAUTHORIZED,
    })
}

#[utoipa::path(
    tag = "admin",
    security(("token" = [])),
)]
#[patch("/admins/:id")]
pub async fn patch_admin_handler(
    auth: AuthorizationService,
    State(service): State<dyn AdminService>,
    Path(id): Path<String>,
    Json(data): Json<PatchAdminDTO>,
) -> Result<Json<AdminDTO>, StatusCode> {
    service
        .patch(id, data, auth.id)
        .await
        .map(Json)
        .map_err(|err| match err {
            BasicError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::UNAUTHORIZED,
        })
}

#[utoipa::path(
    tag = "admin",
    security(("token" = [])),
)]
#[delete("/admins/:id")]
pub async fn delete_admin_handler(
    auth: AuthorizationService,
    State(service): State<dyn AdminService>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    service
        .delete(id, auth.id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| match err {
            BasicError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::UNAUTHORIZED,
        })
}
