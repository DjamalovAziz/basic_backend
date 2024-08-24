use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::Redoc;
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
use crate::middleware::AuthorizationService;

pub async fn run_axum() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let host: String = get_env_or("HOST", "127.0.0.1");
    let port: u16 = get_env_or("PORT", "7003").parse().expect("Invalid PORT value");
    let addr = format!("{}:{}", host, port);

    let serve_from = get_env_or("SERVER_FROM", DEFAULT_SERVE_FROM);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum=info");
    }

    let container = Container::new().await;

    // build our application with a route
    let app = Router::new()
        //
        //admin
        .with_state(container.admin_service.clone())
        //management
        .with_state(container.user_service.clone())
        .with_state(container.relation_service.clone())
        //organization
        .with_state(container.organization_service.clone())
        .with_state(container.branch_service.clone())
        //message
        .with_state(container.telegram_group_service.clone())
        .with_state(container.fcm_subscription_service.clone())
        .with_state(container.subscription_service.clone())
        //common
        .with_state(container.common_service.clone())
        //
        // ==================== ADMIN ==================== //
        .route("/admins/change_password/:id", put(change_admin_password_handler))
        .route("/admins/me", get(get_me_admin_handler))
        .route("/admins/reset_admin_password", put(reset_admin_password_handler))
        .route("/admins/signin", post(signin_admin_handler))
        // Basic
        .route("/admins", get(list_admins_handler))
        .route("/admins/:id", get(get_admin_handler))
        .route("/admins", post(post_admin_handler))
        .route("/admins/:id", patch(patch_admin_handler))
        .route("/admins/:id", delete(delete_admin_handler))
        //
        // ==================== USER ==================== //
        .route("/users/change_password/:id", put(change_user_password_handler))
        .route("/users/reset_user_password", put(reset_user_password_handler))
        .route("/users/signin", post(signin_user_handler))
        .route("/users/signup", post(signup_handler))
        //
        .route("/users", post(post_user_handler))
        .route("/users/:id", get(get_user_handler))
        .route("/users", get(list_users_handler))
        //
        .route("/users/me", delete(delete_self_user_handler))
        .route("/users/me", get(get_self_user_handler))
        .route("/users/me", patch(patch_self_user_handler))
        //
        // ==================== Relation ==================== //
        //
        .route("/relations/:id", delete(delete_relation_handler))
        .route("/relations/invite_to_branch/:id", post(invite_to_branch_handler))
        .route("/relations/my", get(list_my_relations_handler))
        .route("/relations", get(list_relations_handler))
        .route("/relations/:id", patch(patch_handler))
        .route(
            "/relations/invitation_to_branch/:id",
            patch(patch_invitation_to_branch_handler),
        )
        .route(
            "/relations/request_join_to_branch/:id",
            post(request_join_to_branch_handler),
        )
        //
        // ==================== TelegramGroup ==================== //
        .route("/telegram_groups/:id", delete(delete_telegram_group_handler))
        .route("/telegram_groups/:id", get(get_telegram_group_handler))
        .route("/telegram_groups", get(list_telegram_groups_handler))
        .route("/telegram_groups/:id", patch(patch_telegram_group_handler))
        .route("/telegram_groups", post(post_telegram_group_handler))
        //
        // ==================== FCMSUBSCRIPTION ==================== //
        .route("/fcm_subscriptions", post(post_fcm_subscription_handler))
        //
        // ==================== SUBSCRIPTION ==================== //
        .route("/subscriptions", post(post_subscription_handler))
        //
        // ==================== ORGANIZATION ==================== //
        .route("/organizations/:id", delete(delete_organization_handler))
        .route("/organizations/:id", get(get_organization_handler))
        .route("/organizations", get(list_organizations_handler))
        .route("/organizations/:id", patch(patch_organization_handler))
        .route("/organizations", post(post_organization_handler))
        //
        // ==================== BRANCH ==================== //
        .route("/branchs/:id", delete(delete_branch_handler))
        .route("/branchs/:id", get(get_branch_handler))
        .route("/branchs", get(list_branchs_handler))
        .route("/branchs/:id", patch(patch_branch_handler))
        .route("/branchs", post(post_branch_handler))
        //
        // Documentation
        .route("/api-docs/openapi.json", get(|| async { ApiDoc::openapi() }))
        .route(
            "/rapidoc",
            get(|| async { RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()) }),
        )
        .route(
            "/swagger-ui",
            get(|| async { SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()) }),
        )
        .route("/redoc", get(|| async { Redoc::with_url("/redoc", ApiDoc::openapi()) }))
        // Add a fallback service for serving static files
        .fallback_service(
            axum::routing::get_service(tower_http::services::ServeDir::new(serve_from)).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    // run it with hyper on 127.0.0.1:3000
    let addr: SocketAddr = addr.parse().unwrap();
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
