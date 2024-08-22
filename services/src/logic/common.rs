use async_trait::async_trait;
use domain::repositories::admin::AdminTrait;
use domain::repositories::common::CommonRepository;
use domain::repositories::management::{RelationTrait, UserTrait};
use domain::repositories::message::{FCMSubscriptionTrait, SubscriptionTrait, TelegramGroupTrait};
use domain::repositories::organization::{BranchTrait, OrganizationTrait};
use domain::services::common::CommonService;
use std::sync::Arc;
// ==================== USER ==================== //
pub struct CommonServiceImpl {
    pub repository: Arc<dyn CommonRepository>,
    pub user_repository: Arc<dyn UserTrait>,
    pub relation_repository: Arc<dyn RelationTrait>,
    pub organization_repository: Arc<dyn OrganizationTrait>,
    pub branch_repository: Arc<dyn BranchTrait>,
    pub telegram_group_repository: Arc<dyn TelegramGroupTrait>,
    pub subscription_repository: Arc<dyn SubscriptionTrait>,
    pub fcm_subscription_repository: Arc<dyn FCMSubscriptionTrait>,
    pub admin_repository: Arc<dyn AdminTrait>,
}

impl CommonServiceImpl {
    pub fn new(
        repository: Arc<dyn CommonRepository>,
        user_repository: Arc<dyn UserTrait>,
        relation_repository: Arc<dyn RelationTrait>,
        organization_repository: Arc<dyn OrganizationTrait>,
        branch_repository: Arc<dyn BranchTrait>,
        telegram_group_repository: Arc<dyn TelegramGroupTrait>,
        fcm_subscription_repository: Arc<dyn FCMSubscriptionTrait>,
        subscription_repository: Arc<dyn SubscriptionTrait>,
        admin_repository: Arc<dyn AdminTrait>,
    ) -> Self {
        CommonServiceImpl {
            repository,
            user_repository,
            relation_repository,
            organization_repository,
            branch_repository,
            telegram_group_repository,
            fcm_subscription_repository,
            subscription_repository,
            admin_repository,
        }
    }
}

#[async_trait]
impl CommonService for CommonServiceImpl {}
