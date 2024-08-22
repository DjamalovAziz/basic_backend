use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use super::controllers::admin::{
    __path_change_admin_password_handler, __path_delete_admin_handler, __path_get_admin_handler,
    __path_get_me_admin_handler, __path_list_admins_handler, __path_patch_admin_handler, __path_post_admin_handler,
    __path_reset_admin_password_handler, __path_signin_admin_handler,
};
use super::controllers::management::{
    __path_change_user_password_handler, __path_delete_relation_handler, __path_delete_self_user_handler,
    __path_get_self_user_handler, __path_get_user_handler, __path_invite_to_branch_handler,
    __path_list_my_relations_handler, __path_list_relations_handler, __path_list_users_handler, __path_patch_handler,
    __path_patch_invitation_to_branch_handler, __path_patch_self_user_handler, __path_post_user_handler,
    __path_request_join_to_branch_handler, __path_reset_user_password_handler, __path_signin_user_handler,
    __path_signup_handler,
};
use super::controllers::message::{
    __path_delete_telegram_group_handler, __path_get_telegram_group_handler, __path_list_telegram_groups_handler,
    __path_patch_telegram_group_handler, __path_post_fcm_subscription_handler, __path_post_subscription_handler,
    __path_post_telegram_group_handler,
};
use super::controllers::organization::{
    __path_delete_branch_handler, __path_delete_organization_handler, __path_get_branch_handler,
    __path_get_organization_handler, __path_list_branchs_handler, __path_list_organizations_handler,
    __path_patch_branch_handler, __path_patch_organization_handler, __path_post_branch_handler,
    __path_post_organization_handler,
};

use common::enums::{AdminRoleType, ProductType, RelationType, UserRoleType};
use domain::dto::admin::{AdminDTO, PatchAdminDTO, PostAdminDTO, PutAdminPasswordDTO, SignInAdminDTO};
use domain::dto::management::{
    InviteToBranchDTO, MultipartPatchUserImageDTO, MultipartPostUserImageDTO, PatchInviteToBranchDTO, PatchRelationDTO,
    PatchUserDTO, PhoneNumberDTO, PostUserDTO, PutUserPasswordDTO, RelationDTO, RequestJoinToBranchDTO, SignInUserDTO,
    SignUpDTO, UserDTO,
};
use domain::dto::message::{
    FCMSubscriptionDTO, PatchTelegramGroupDTO, PostFCMSubscriptionDTO, PostKeysDTO, PostSubscriptionDTO,
    PostSubscriptionFieldDTO, PostTelegramGroupDTO, SubscriptionDTO, TelegramGroupDTO,
};
use domain::dto::organization::{
    BranchDTO, OrganizationDTO, PatchBranchDTO, PatchOrganizationDTO, PostBranchDTO, PostOrganizationDTO,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // ==================== ADMIN ==================== //
        change_admin_password_handler, delete_admin_handler, get_admin_handler,
        get_me_admin_handler, list_admins_handler, patch_admin_handler,
        post_admin_handler, reset_admin_password_handler, signin_admin_handler,

        // ==================== MANAGEMENT ==================== //
        change_user_password_handler, delete_relation_handler, delete_self_user_handler,
        get_self_user_handler, get_user_handler, invite_to_branch_handler,
        list_my_relations_handler, list_relations_handler, list_users_handler,
        patch_handler, patch_invitation_to_branch_handler, patch_self_user_handler,
        post_user_handler, request_join_to_branch_handler, reset_user_password_handler,
        signin_user_handler, signup_handler,

        // ==================== MESSAGE ==================== //
        delete_telegram_group_handler, get_telegram_group_handler,
        list_telegram_groups_handler, patch_telegram_group_handler,
        post_fcm_subscription_handler, post_subscription_handler,
        post_telegram_group_handler,

        // ==================== ORGANIZATION ==================== //
        delete_branch_handler, delete_organization_handler, get_branch_handler,
        get_organization_handler, list_branchs_handler, list_organizations_handler,
        patch_branch_handler, patch_organization_handler, post_branch_handler,
        post_organization_handler,
        // ==================== COMMON ==================== //
    ),
    components(schemas(
        // ==================== ADMIN ==================== //
        AdminDTO, PatchAdminDTO, PostAdminDTO, PutAdminPasswordDTO, SignInAdminDTO,

        // ==================== MANAGEMENT ==================== //
        InviteToBranchDTO, MultipartPatchUserImageDTO, MultipartPostUserImageDTO, PatchInviteToBranchDTO,
        PatchRelationDTO, PatchUserDTO, PhoneNumberDTO, PostUserDTO, PutUserPasswordDTO, RelationDTO,
        RequestJoinToBranchDTO, SignInUserDTO, SignUpDTO, UserDTO,

        // ==================== MESSAGE ==================== //
        FCMSubscriptionDTO, PatchTelegramGroupDTO, PostFCMSubscriptionDTO, PostKeysDTO, PostSubscriptionDTO,
        PostSubscriptionFieldDTO, PostTelegramGroupDTO, SubscriptionDTO, TelegramGroupDTO,

        // ==================== ORGANIZATION ==================== //
        BranchDTO, OrganizationDTO, PatchBranchDTO, PatchOrganizationDTO, PostBranchDTO, PostOrganizationDTO,

        // ==================== COMMON ==================== //

        //enums
        AdminRoleType, ProductType, RelationType, UserRoleType
    )),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().expect("SecurityAddon Error");
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
