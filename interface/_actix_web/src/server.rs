use actix_cors::Cors;
use actix_files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable as RedocServable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use common::constant::{DEFAULT_MOUNT_PATH, DEFAULT_SERVE_FROM};
use common::functions::get_env_or;
use container::Container;

use crate::api_docs::ApiDoc;
use crate::controllers::admin::{
    change_admin_password_handler, delete_admin_handler, get_admin_handler, get_me_admin_handler, list_admins_handler,
    patch_admin_handler, post_admin_handler, reset_admin_password_handler, signin_admin_handler,
};
use crate::controllers::management::{
    change_user_password_handler, delete_relation_handler, delete_self_user_handler, get_self_user_handler,
    get_user_handler, invite_to_branch_handler, list_my_relations_handler, list_relations_handler, list_users_handler,
    patch_handler, patch_invitation_to_branch_handler, patch_self_user_handler, post_user_handler,
    request_join_to_branch_handler, reset_user_password_handler, signin_user_handler, signup_handler,
};
use crate::controllers::message::{
    delete_telegram_group_handler, get_telegram_group_handler, list_telegram_groups_handler,
    patch_telegram_group_handler, post_fcm_subscription_handler, post_subscription_handler,
    post_telegram_group_handler,
};
use crate::controllers::organization::{
    delete_branch_handler, delete_organization_handler, get_branch_handler, get_organization_handler,
    list_branchs_handler, list_organizations_handler, patch_branch_handler, patch_organization_handler,
    post_branch_handler, post_organization_handler,
};

pub async fn run_actix_web() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host: String = get_env_or("HOST", "0.0.0.0");
    let port: u16 = get_env_or("PORT", "7003").parse().expect("Invalid PORT value");
    let addr = format!("{}:{}", host, port);

    let serve_from = get_env_or("SERVER_FROM", DEFAULT_SERVE_FROM);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let container = Container::new().await;
    HttpServer::new(move || {
        App::new()
            //
            //admin
            .app_data(web::Data::from(container.admin_service.clone()))
            //management
            .app_data(web::Data::from(container.user_service.clone()))
            .app_data(web::Data::from(container.relation_service.clone()))
            //organization
            .app_data(web::Data::from(container.organization_service.clone()))
            .app_data(web::Data::from(container.branch_service.clone()))
            //message
            .app_data(web::Data::from(container.telegram_group_service.clone()))
            .app_data(web::Data::from(container.fcm_subscription_service.clone()))
            .app_data(web::Data::from(container.subscription_service.clone()))
            //common
            .app_data(web::Data::from(container.common_service.clone()))
            // wrap
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            // actix-files
            .service(actix_files::Files::new(DEFAULT_MOUNT_PATH, serve_from.as_str()))
            //
            // ==================== ADMIN ==================== //
            .service(change_admin_password_handler)
            .service(get_me_admin_handler)
            .service(reset_admin_password_handler)
            .service(signin_admin_handler)
            // Basic
            .service(delete_admin_handler)
            .service(get_admin_handler)
            .service(list_admins_handler)
            .service(patch_admin_handler)
            .service(post_admin_handler)
            //
            // ==================== USER ==================== //
            .service(change_user_password_handler)
            .service(reset_user_password_handler)
            .service(signin_user_handler)
            .service(signup_handler)
            //
            .service(post_user_handler)
            .service(get_user_handler)
            .service(list_users_handler)
            //
            .service(delete_self_user_handler)
            .service(get_self_user_handler)
            .service(patch_self_user_handler)
            //
            // ==================== Relation ==================== //
            //
            .service(delete_relation_handler)
            .service(invite_to_branch_handler)
            .service(list_my_relations_handler)
            .service(list_relations_handler)
            .service(patch_handler)
            .service(patch_invitation_to_branch_handler)
            .service(request_join_to_branch_handler)
            //
            // ==================== TelegramGroup ==================== //
            .service(delete_telegram_group_handler)
            .service(get_telegram_group_handler)
            .service(list_telegram_groups_handler)
            .service(patch_telegram_group_handler)
            .service(post_telegram_group_handler)
            //
            // ==================== FCMSUBSCRIPTION ==================== //
            .service(post_fcm_subscription_handler)
            //
            // ==================== SUBSCRIPTION ==================== //
            .service(post_subscription_handler)
            //
            // ==================== ORGANIZATION ==================== //
            .service(delete_organization_handler)
            .service(get_organization_handler)
            .service(list_organizations_handler)
            .service(patch_organization_handler)
            .service(post_organization_handler)
            //
            // ==================== BRANCH ==================== //
            .service(delete_branch_handler)
            .service(get_branch_handler)
            .service(list_branchs_handler)
            .service(patch_branch_handler)
            .service(post_branch_handler)
            //
            // Documentation
            .service(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc{_:.*}"))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .service(Redoc::with_url("/redoc{_:.*}", ApiDoc::openapi()))
            .service(Scalar::with_url("/scalar{_:.*}", ApiDoc::openapi()))
    })
    .bind(&addr)?
    .run()
    .await?;
    Ok(())
}
