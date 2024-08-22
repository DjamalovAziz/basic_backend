use async_trait::async_trait;

use std::sync::Arc;

use domain::repositories::admin::AdminTrait;
use domain::repositories::common::CommonRepository;
use domain::repositories::management::{RelationTrait, UserTrait};
use domain::repositories::message::{FCMSubscriptionTrait, SubscriptionTrait, TelegramGroupTrait};
use domain::repositories::organization::{BranchTrait, OrganizationTrait};
use domain::services::admin::AdminService;
use domain::services::common::CommonService;
use domain::services::management::{RelationService, UserService};
use domain::services::message::{FCMSubscriptionService, SubscriptionService, TelegramGroupService};
use domain::services::organization::{BranchService, OrganizationService};
// use postgresql::repositories::admin::AdminSurrealRepository;
use _surrealdb::pool::surrealdb_pool;
use _surrealdb::repositories::admin::AdminSurrealRepository;
use _surrealdb::repositories::common::CommonSurrealRepository;
use _surrealdb::repositories::management::{RelationSurrealRepository, UserSurrealRepository};
use _surrealdb::repositories::message::{
    FCMSubscriptionSurrealRepository, SubscriptionSurrealRepository, TelegramGroupSurrealRepository,
};
use _surrealdb::repositories::organization::{BranchSurrealRepository, OrganizationSurrealRepository};
use services::logic::admin::AdminServiceImpl;
use services::logic::common::CommonServiceImpl;
use services::logic::management::{RelationServiceImpl, UserServiceImpl};
use services::logic::message::{FCMSubscriptionServiceImpl, SubscriptionServiceImpl, TelegramGroupServiceImpl};
use services::logic::organization::{BranchServiceImpl, OrganizationServiceImpl};

#[async_trait]
pub trait AsyncDefault {
    async fn default() -> Self;
}

pub struct Container {
    pub admin_service: Arc<dyn AdminService>,

    pub user_service: Arc<dyn UserService>,
    pub relation_service: Arc<dyn RelationService>,

    pub organization_service: Arc<dyn OrganizationService>,
    pub branch_service: Arc<dyn BranchService>,

    pub telegram_group_service: Arc<dyn TelegramGroupService>,
    pub fcm_subscription_service: Arc<dyn FCMSubscriptionService>,
    pub subscription_service: Arc<dyn SubscriptionService>,

    pub common_service: Arc<dyn CommonService>,
}

impl Container {
    pub async fn new() -> Self {
        let db = Arc::new(surrealdb_pool().await.expect("Database connection error!"));

        // ==================== Repository ==================== //
        //  ADMIN  //
        let admin_repository: Arc<dyn AdminTrait> = Arc::new(AdminSurrealRepository::new(Arc::clone(&db)));

        //  MANAGEMENT  //
        let user_repository: Arc<dyn UserTrait> = Arc::new(UserSurrealRepository::new(Arc::clone(&db)));

        let relation_repository: Arc<dyn RelationTrait> = Arc::new(RelationSurrealRepository::new(Arc::clone(&db)));

        //  ORGANIZATION  //
        let organization_repository: Arc<dyn OrganizationTrait> =
            Arc::new(OrganizationSurrealRepository::new(Arc::clone(&db)));
        let branch_repository: Arc<dyn BranchTrait> = Arc::new(BranchSurrealRepository::new(Arc::clone(&db)));

        //  MESSAGE  //
        let telegram_group_repository: Arc<dyn TelegramGroupTrait> =
            Arc::new(TelegramGroupSurrealRepository::new(Arc::clone(&db)));
        let fcm_subscription_repository: Arc<dyn FCMSubscriptionTrait> =
            Arc::new(FCMSubscriptionSurrealRepository::new(Arc::clone(&db)));
        let subscription_repository: Arc<dyn SubscriptionTrait> =
            Arc::new(SubscriptionSurrealRepository::new(Arc::clone(&db)));

        //  COMMON  //
        let common_repository: Arc<dyn CommonRepository> = Arc::new(CommonSurrealRepository::new(Arc::clone(&db)));

        // ==================== Service ==================== //

        let admin_service = Arc::new(AdminServiceImpl {
            repository: admin_repository.clone(),
        });

        //  USER  //
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repository.clone(),
        });

        //  ORGANIZATION  //
        let organization_service = Arc::new(OrganizationServiceImpl {
            repository: organization_repository.clone(),
            branch_repository: branch_repository.clone(),
            user_repository: user_repository.clone(),
            relation_repository: relation_repository.clone(),
        });

        //  BRANCH  //
        let branch_service = Arc::new(BranchServiceImpl {
            repository: branch_repository.clone(),
            user_repository: user_repository.clone(),
            relation_repository: relation_repository.clone(),
        });

        //  Relation  //
        let relation_service = Arc::new(RelationServiceImpl {
            repository: relation_repository.clone(),
            user_repository: user_repository.clone(),
        });

        //  TelegramGroup  //
        let telegram_group_service = Arc::new(TelegramGroupServiceImpl {
            repository: telegram_group_repository.clone(),
            relation_repository: relation_repository.clone(),
        });

        //  FCMSUBSCRIPTION  //
        let fcm_subscription_service = Arc::new(FCMSubscriptionServiceImpl {
            repository: fcm_subscription_repository.clone(),
            relation_repository: relation_repository.clone(),
        });

        //  SUBSCRIPTION  //
        let subscription_service = Arc::new(SubscriptionServiceImpl {
            repository: subscription_repository.clone(),
            relation_repository: relation_repository.clone(),
        });

        //  COMMON  //
        let common_service = Arc::new(CommonServiceImpl {
            repository: common_repository,
            user_repository,
            relation_repository,
            organization_repository,
            branch_repository,
            telegram_group_repository,
            fcm_subscription_repository,
            subscription_repository,
            admin_repository,
        });

        Container {
            admin_service,
            user_service,
            relation_service,
            organization_service,
            branch_service,
            telegram_group_service,
            fcm_subscription_service,
            subscription_service,
            common_service,
        }
    }
}

#[async_trait]
impl AsyncDefault for Container {
    async fn default() -> Self {
        Self::new().await
    }
}
