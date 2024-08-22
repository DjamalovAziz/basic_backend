use actix_multipart::Multipart;
use actix_web::{delete, get, patch, post, put, web, Error as ActixWebError, HttpResponse, Result};
use futures_util::{StreamExt, TryStreamExt};
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use uuid::Uuid;

use common::errors::BasicError;
use domain::dto::management::{
    InviteToBranchDTO, PatchInviteToBranchDTO, PatchRelationDTO, PatchUserDTO, PhoneNumberDTO, PostUserDTO,
    PutUserPasswordDTO, RelationDTO, RequestJoinToBranchDTO, SignInUserDTO, SignUpDTO, UserDTO,
};
use domain::models::management::{ServicePatchUserImage, ServicePostUserImage};
use domain::repositories::management::{RelationQueryParams, UserQueryParams};
use domain::repositories::repository::{ResultPaging, Token};
use domain::services::management::{RelationService, UserService};

use crate::middleware::{AuthorizationService, XBranchService, XOrganizationService};

// ==================== USER ==================== //

#[utoipa::path(security(("token" = [])))]
#[put("/users/change_password")]
pub async fn change_user_password_handler(
    auth: AuthorizationService,
    service: web::Data<dyn UserService>,
    data: web::Json<PutUserPasswordDTO>,
) -> Result<web::Json<String>, ActixWebError> {
    Ok(web::Json(service.change_password(data.into_inner(), auth.id).await?))
}

#[utoipa::path()]
#[put("/users/reset_user_password")]
pub async fn reset_user_password_handler(
    service: web::Data<dyn UserService>,
    data: web::Json<PhoneNumberDTO>,
) -> Result<web::Json<String>, ActixWebError> {
    Ok(web::Json(service.reset_user_password(&data.phone_number).await?))
}

#[utoipa::path]
#[post("/users/signin")]
pub async fn signin_user_handler(
    service: web::Data<dyn UserService>,
    data: web::Json<SignInUserDTO>,
) -> Result<web::Json<Token>, ActixWebError> {
    Ok(web::Json(service.signin_user(data.into_inner()).await?))
}

#[utoipa::path]
#[post("/users/signup")]
pub async fn signup_handler(
    service: web::Data<dyn UserService>,
    data: web::Json<SignUpDTO>,
) -> Result<web::Json<Token>, ActixWebError> {
    Ok(web::Json(service.signup(data.into_inner()).await?))
}

//
#[utoipa::path(
    security(("token" = [])),
    request_body(content_type = "multipart/form-data", content = MultipartPostUserImageDTO)
)]
#[post("/users")]
pub async fn post_user_handler(
    auth: AuthorizationService,
    service: web::Data<dyn UserService>,
    mut payload: Multipart,
) -> Result<web::Json<UserDTO>, ActixWebError> {
    let mut user: Option<PostUserDTO> = None;
    let mut image_destination: Option<String> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        if field.name() == "user" {
            let mut user_data = Vec::new();
            while let Some(chunk) = field.next().await {
                user_data.extend_from_slice(&chunk?);
            }
            user = serde_json::from_slice(&user_data).ok();
            continue;
        } else if field.name() == "image" {
            if let Some(filename) = field.content_disposition().get_filename() {
                let image_destination_data = format!("{}{}-{}", "./media/images/avatars/", Uuid::new_v4(), filename);

                let mut saved_file = fs::File::create(&image_destination_data).await?;
                while let Ok(Some(chunk)) = field.try_next().await {
                    saved_file.write_all(&chunk).await?;
                }

                image_destination = Some(image_destination_data);
            }
            continue;
        }
    }

    if let (Some(user), image_destination) = (user, image_destination) {
        let form_data = ServicePostUserImage {
            user,
            image_destination,
        };
        Ok(web::Json(service.create(form_data, auth.id).await?.into()))
    } else {
        Err(BasicError::bad_request_error(String::from("Bad request interface!")).into())
    }
}

#[utoipa::path(security(("token" = [])))]
#[get("/users/{id}")]
pub async fn get_user_handler(
    auth: AuthorizationService,
    service: web::Data<dyn UserService>,
    params: web::Path<String>,
) -> Result<web::Json<UserDTO>, ActixWebError> {
    Ok(web::Json(service.get(params.into_inner(), auth.id).await?.into()))
}

#[utoipa::path(security(("token" = [])), params(UserQueryParams))]
#[get("/users")]
pub async fn list_users_handler(
    auth: AuthorizationService,
    service: web::Data<dyn UserService>,
    params: web::Query<UserQueryParams>,
) -> Result<web::Json<ResultPaging<UserDTO>>, ActixWebError> {
    Ok(web::Json(
        service.list(params.into_inner(), auth.id).await?.paging_from(),
    ))
}

//
#[utoipa::path(security(("token" = [])))]
#[delete("/users")]
pub async fn delete_self_user_handler(
    auth: AuthorizationService,
    service: web::Data<dyn UserService>,
) -> Result<HttpResponse, ActixWebError> {
    service.delete_self(auth.id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(security(("token" = [])))]
#[get("/users/me")]
pub async fn get_self_user_handler(
    auth: AuthorizationService,
    service: web::Data<dyn UserService>,
) -> Result<web::Json<UserDTO>, ActixWebError> {
    Ok(web::Json(service.get_self(auth.id).await?.into()))
}

#[utoipa::path(
    security(("token" = [])),
    request_body(content_type = "multipart/form-data", content = MultipartPatchUserImageDTO)
)]
#[patch("/users")]
pub async fn patch_self_user_handler(
    auth: AuthorizationService,
    service: web::Data<dyn UserService>,
    mut payload: Multipart,
) -> Result<web::Json<UserDTO>, ActixWebError> {
    let mut user: Option<PatchUserDTO> = None;
    let mut image_destination: Option<String> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        if field.name() == "user" {
            let mut user_data = Vec::new();
            while let Some(chunk) = field.next().await {
                user_data.extend_from_slice(&chunk?);
            }
            user = serde_json::from_slice(&user_data).ok();
            continue;
        } else if field.name() == "image" {
            if let Some(filename) = field.content_disposition().get_filename() {
                let image_destination_data = format!("{}{}-{}", "./media/images/avatars/", Uuid::new_v4(), filename);

                let mut saved_file = fs::File::create(&image_destination_data).await?;
                while let Ok(Some(chunk)) = field.try_next().await {
                    saved_file.write_all(&chunk).await?;
                }

                image_destination = Some(image_destination_data);
            }
            continue;
        }
    }

    if let (Some(user), image_destination) = (user, image_destination) {
        let form_data = ServicePatchUserImage {
            user,
            image_destination,
        };
        Ok(web::Json(service.patch_self(form_data, auth.id).await?.into()))
    } else {
        Err(BasicError::bad_request_error(String::from("Bad request interface!")).into())
    }
}

// ==================== Relation ==================== //
#[utoipa::path(security(("token" = [])))]
#[delete("/relations/{id}")]
pub async fn delete_relation_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn RelationService>,
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
#[post("/invite_to_branch")]
pub async fn invite_to_branch_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn RelationService>,
    data: web::Json<InviteToBranchDTO>,
) -> Result<web::Json<RelationDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .invite_to_branch(
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
#[get("/relations/my_relations")]
pub async fn list_my_relations_handler(
    auth: AuthorizationService,
    service: web::Data<dyn RelationService>,
) -> Result<web::Json<Vec<RelationDTO>>, ActixWebError> {
    Ok(web::Json(
        service
            .list_my_relations(auth.id)
            .await?
            .into_iter()
            .map(|relation| relation.into())
            .collect(),
    ))
}

#[utoipa::path(security(("token" = [])))]
#[patch("/relations/{id}")]
pub async fn patch_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn RelationService>,
    params: web::Path<String>,
    data: web::Json<PatchRelationDTO>,
) -> Result<web::Json<RelationDTO>, ActixWebError> {
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
#[patch("/relations/{id}")]
pub async fn patch_invitation_to_branch_handler(
    auth: AuthorizationService,
    x_organization: XOrganizationService,
    x_branch: XBranchService,
    service: web::Data<dyn RelationService>,
    params: web::Path<String>,
    data: web::Json<PatchInviteToBranchDTO>,
) -> Result<web::Json<RelationDTO>, ActixWebError> {
    Ok(web::Json(
        service
            .patch_invitation_to_branch(
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
#[post("/request_join_to_branch")]
pub async fn request_join_to_branch_handler(
    auth: AuthorizationService,
    service: web::Data<dyn RelationService>,
    data: web::Json<RequestJoinToBranchDTO>,
) -> Result<web::Json<RelationDTO>, ActixWebError> {
    Ok(web::Json(
        service.request_join_to_branch(data.into_inner(), auth.id).await?.into(),
    ))
}

#[utoipa::path(security(("token" = [])), params(RelationQueryParams))]
#[get("/relations")]
pub async fn list_relations_handler(
    auth: AuthorizationService,
    service: web::Data<dyn RelationService>,
    params: web::Query<RelationQueryParams>,
) -> Result<web::Json<ResultPaging<RelationDTO>>, ActixWebError> {
    Ok(web::Json(
        service.list(params.into_inner(), auth.id).await?.paging_from(),
    ))
}
