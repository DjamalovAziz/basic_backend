use actix_web::{delete, get, patch, post, web, Error as ActixWebError, HttpResponse};

use crate::middleware::{AuthorizationService, XBranchService, XOrganizationService};

use domain::dto::message::{
    FCMSubscriptionDTO, PatchTelegramGroupDTO, PostFCMSubscriptionDTO, PostSubscriptionDTO, PostTelegramGroupDTO,
    SubscriptionDTO, TelegramGroupDTO,
};
use domain::services::message::{FCMSubscriptionService, SubscriptionService, TelegramGroupService};

// ==================== TelegramGroup ==================== //
//Basic
#[utoipa::path(security(("token" = [])))]
#[delete("/telegram_groups/{id}")]
pub async fn delete_telegram_group_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn TelegramGroupService>,
    params: web::Path<String>,
) -> Result<HttpResponse, ActixWebError> {
    service
        .delete(
            params.into_inner(),
            auth.id,
            x_organization.organization_id,
            x_branch.branch_id,
        )
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(security(("token" = [])))]
#[get("/telegram_groups/{id}")]
pub async fn get_telegram_group_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn TelegramGroupService>,
    params: web::Path<String>,
) -> Result<web::Json<TelegramGroupDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .get(
                params.into_inner(),
                auth.id,
                x_organization.organization_id,
                x_branch.branch_id,
            )
            .await?
            .into(),
    ))
}

#[utoipa::path(security(("token" = [])))]
#[patch("/telegram_groups/{id}")]
pub async fn patch_telegram_group_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn TelegramGroupService>,
    params: web::Path<String>,
    data: web::Json<PatchTelegramGroupDTO>,
) -> Result<web::Json<TelegramGroupDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .patch(
                params.into_inner(),
                data.into_inner(),
                auth.id,
                x_organization.organization_id,
                x_branch.branch_id,
            )
            .await?
            .into(),
    ))
}

#[utoipa::path(security(("token" = [])))]
#[post("/telegram_groups")]
pub async fn post_telegram_group_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn TelegramGroupService>,
    data: web::Json<PostTelegramGroupDTO>,
) -> Result<web::Json<TelegramGroupDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .create(
                data.into_inner(),
                auth.id,
                x_organization.organization_id,
                x_branch.branch_id,
            )
            .await?
            .into(),
    ))
}

#[utoipa::path(security(("token" = [])))]
#[get("/telegram_groups")]
pub async fn list_telegram_groups_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    service: web::Data<dyn TelegramGroupService>,
) -> Result<web::Json<Vec<TelegramGroupDTO>>, ActixWebError> {
    Ok(web::Json(
        service
            .list(auth.id, x_organization.organization_id)
            .await?
            .into_iter()
            .map(TelegramGroupDTO::from)
            .collect(),
    ))
}

// ==================== FCM_NOTIFICATION ==================== //
#[utoipa::path(security(("token" = [])))]
#[post("/fcm_subscriptions")]
pub async fn post_fcm_subscription_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn FCMSubscriptionService>,
    data: web::Json<PostFCMSubscriptionDTO>,
) -> Result<web::Json<FCMSubscriptionDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .create(
                data.into_inner(),
                auth.id,
                x_organization.organization_id,
                x_branch.branch_id,
            )
            .await?
            .into(),
    ))
}

// ==================== Subscription ==================== //
#[utoipa::path(security(("token" = [])))]
#[post("/subscriptions")]
pub async fn post_subscription_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn SubscriptionService>,
    data: web::Json<PostSubscriptionDTO>,
) -> Result<web::Json<SubscriptionDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .create(
                data.into_inner(),
                auth.id,
                x_organization.organization_id,
                x_branch.branch_id,
            )
            .await?
            .into(),
    ))
}
