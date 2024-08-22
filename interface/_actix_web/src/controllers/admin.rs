use actix_web::{delete, get, patch, post, put, web, Error as ActixWebError, HttpResponse, Result};
use common::errors::BasicError;

use crate::middleware::AuthorizationService;

use domain::dto::admin::{AdminDTO, PatchAdminDTO, PostAdminDTO, PutAdminPasswordDTO, SignInAdminDTO};
use domain::dto::management::PhoneNumberDTO;
use domain::repositories::admin::AdminQueryParams;
use domain::repositories::repository::{ResultPaging, Token};
use domain::services::admin::AdminService;

// ==================== ADMIN ==================== //
#[utoipa::path(tag = "admin", security(("token" = [])))]
#[put("/admins/change_password/{id}")]
pub async fn change_admin_password_handler(
    auth: AuthorizationService,
    service: web::Data<dyn AdminService>,
    params: web::Path<String>,
    data: web::Json<PutAdminPasswordDTO>,
) -> Result<web::Json<String>, ActixWebError> {
    service
        .change_password(params.into_inner(), data.into_inner(), auth.id)
        .await
        .map_err(BasicError::from)?; // Преобразуем ошибки в BasicError и возвращаем их

    Ok(web::Json("Password changed successfully".to_string()))
}

#[utoipa::path(tag = "admin", security(("token" = [])))]
#[get("/admins/me")]
pub async fn get_me_admin_handler(
    auth: AuthorizationService,
    service: web::Data<dyn AdminService>,
) -> Result<web::Json<AdminDTO>, ActixWebError> {
    Ok(web::Json(service.get_me_admin(auth.id).await?.into()))
}

#[utoipa::path(tag = "admin")]
#[put("/admins/reset_admin_password")]
pub async fn reset_admin_password_handler(
    service: web::Data<dyn AdminService>,
    data: web::Json<PhoneNumberDTO>,
) -> Result<web::Json<String>, ActixWebError> {
    Ok(web::Json(service.reset_admin_password(&data.phone_number).await?))
}

#[utoipa::path(tag = "admin")]
#[post("/admins/signin")]
pub async fn signin_admin_handler(
    service: web::Data<dyn AdminService>,
    data: web::Json<SignInAdminDTO>,
) -> Result<web::Json<Token>, ActixWebError> {
    Ok(web::Json(service.signin_admin(data.into_inner()).await?))
}

// BASIC
#[utoipa::path(tag = "admin", security(("token" = [])), params(AdminQueryParams))]
#[get("/admins")]
pub async fn list_admins_handler(
    auth: AuthorizationService,
    service: web::Data<dyn AdminService>,
    params: web::Query<AdminQueryParams>,
) -> Result<web::Json<ResultPaging<AdminDTO>>, ActixWebError> {
    Ok(web::Json(
        service.list(params.into_inner(), auth.id).await?.paging_from(),
    ))
}

#[utoipa::path(tag = "admin", security(("token" = [])))]
#[get("/admins/{id}")]
pub async fn get_admin_handler(
    auth: AuthorizationService,
    service: web::Data<dyn AdminService>,
    params: web::Path<String>,
) -> Result<web::Json<AdminDTO>, ActixWebError> {
    Ok(web::Json(service.get(params.into_inner(), auth.id).await?.into()))
}

#[utoipa::path(tag = "admin", security(("token" = [])))]
#[post("/admins")]
pub async fn post_admin_handler(
    auth: AuthorizationService,
    service: web::Data<dyn AdminService>,
    data: web::Json<PostAdminDTO>,
) -> Result<web::Json<AdminDTO>, ActixWebError> {
    Ok(web::Json(service.create(data.into_inner(), auth.id).await?.into()))
}

#[utoipa::path(tag = "admin", security(("token" = [])))]
#[patch("/admins/{id}")]
pub async fn patch_admin_handler(
    auth: AuthorizationService,
    service: web::Data<dyn AdminService>,
    params: web::Path<String>,
    data: web::Json<PatchAdminDTO>,
) -> Result<web::Json<AdminDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .patch(params.into_inner(), data.into_inner(), auth.id)
            .await?
            .into(),
    ))
}

#[utoipa::path(tag = "admin", security(("token" = [])))]
#[delete("/admins/{id}")]
pub async fn delete_admin_handler(
    auth: AuthorizationService,
    service: web::Data<dyn AdminService>,
    params: web::Path<String>,
) -> Result<HttpResponse, ActixWebError> {
    service.delete(params.into_inner(), auth.id).await?;
    Ok(HttpResponse::NoContent().finish())
}
