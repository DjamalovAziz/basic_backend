use actix_web::{delete, get, patch, post, web, Error as ActixWebError, HttpResponse, Result};

use domain::dto::organization::{
    BranchDTO, OrganizationDTO, PatchBranchDTO, PatchOrganizationDTO, PostBranchDTO, PostOrganizationDTO,
};
use domain::repositories::organization::{BranchQueryParams, OrganizationQueryParams};
use domain::repositories::repository::ResultPaging;
use domain::services::organization::{BranchService, OrganizationService};

use crate::middleware::{AuthorizationService, XOrganizationService};

// ==================== ORGANIZATION ==================== //

#[utoipa::path(security(("token" = [])))]
#[delete("/organizations/{id}")]
pub async fn delete_organization_handler(
    auth: AuthorizationService,
    service: web::Data<dyn OrganizationService>,
    params: web::Path<String>,
) -> Result<HttpResponse, ActixWebError> {
    service.delete(params.into_inner(), auth.id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(security(("token" = [])))]
#[get("/organizations/{id}")]
pub async fn get_organization_handler(
    auth: AuthorizationService,
    service: web::Data<dyn OrganizationService>,
    params: web::Path<String>,
) -> Result<web::Json<OrganizationDTO>, ActixWebError> {
    Ok(web::Json(service.get(params.into_inner(), auth.id).await?.into()))
}

#[utoipa::path(security(("token" = [])), params(OrganizationQueryParams))]
#[get("/organizations")]
pub async fn list_organizations_handler(
    auth: AuthorizationService,
    service: web::Data<dyn OrganizationService>,
    params: web::Query<OrganizationQueryParams>,
) -> Result<web::Json<ResultPaging<OrganizationDTO>>, ActixWebError> {
    Ok(web::Json(
        service.list(params.into_inner(), auth.id).await?.paging_from(),
    ))
}

#[utoipa::path(security(("token" = [])))]
#[patch("/organizations/{id}")]
pub async fn patch_organization_handler(
    auth: AuthorizationService,
    service: web::Data<dyn OrganizationService>,
    params: web::Path<String>,
    data: web::Json<PatchOrganizationDTO>,
) -> Result<web::Json<OrganizationDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .patch(params.into_inner(), data.into_inner(), auth.id)
            .await?
            .into(),
    ))
}

#[utoipa::path(security(("token" = [])))]
#[post("/organizations")]
pub async fn post_organization_handler(
    auth: AuthorizationService,
    service: web::Data<dyn OrganizationService>,
    data: web::Json<PostOrganizationDTO>,
) -> Result<web::Json<OrganizationDTO>, ActixWebError> {
    Ok(web::Json(service.create(data.into_inner(), auth.id).await?.into()))
}

// ==================== BRANCH ==================== //
#[utoipa::path(security(("token" = [])))]
#[delete("/branchs/{id}")]
pub async fn delete_branch_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    service: web::Data<dyn BranchService>,
    params: web::Path<String>,
) -> Result<HttpResponse, ActixWebError> {
    service
        .delete(params.into_inner(), auth.id, x_organization.organization_id)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(security(("token" = [])), params(BranchQueryParams))]
#[get("/branchs")]
pub async fn list_branchs_handler(
    auth: AuthorizationService,
    service: web::Data<dyn BranchService>,
    params: web::Query<BranchQueryParams>,
) -> Result<web::Json<ResultPaging<BranchDTO>>, ActixWebError> {
    Ok(web::Json(
        service.list(params.into_inner(), auth.id).await?.paging_from(),
    ))
}

#[utoipa::path(security(("token" = [])))]
#[get("/branchs/{id}")]
pub async fn get_branch_handler(
    auth: AuthorizationService,
    service: web::Data<dyn BranchService>,
    params: web::Path<String>,
) -> Result<web::Json<BranchDTO>, ActixWebError> {
    Ok(web::Json(service.get(params.into_inner(), auth.id).await?.into()))
}

#[utoipa::path(security(("token" = [])))]
#[patch("/branchs/{id}")]
pub async fn patch_branch_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    service: web::Data<dyn BranchService>,
    params: web::Path<String>,
    data: web::Json<PatchBranchDTO>,
) -> Result<web::Json<BranchDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .patch(
                params.into_inner(),
                data.into_inner(),
                auth.id,
                x_organization.organization_id,
            )
            .await?
            .into(),
    ))
}

#[utoipa::path(security(("token" = [])))]
#[post("/branchs")]
pub async fn post_branch_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    service: web::Data<dyn BranchService>,
    data: web::Json<PostBranchDTO>,
) -> Result<web::Json<BranchDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .create(data.into_inner(), auth.id, x_organization.organization_id)
            .await?
            .into(),
    ))
}
