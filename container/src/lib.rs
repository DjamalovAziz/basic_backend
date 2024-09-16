use async_trait::async_trait;
use postgresql::pool::postgresql_pool;
use postgresql::repositories::admin::AdminPostgresqlRepository;

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
    pub admin_service_surrealdb: Arc<dyn AdminService>,
    pub admin_service_postgresql: Arc<dyn AdminService>,

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
        let surrealdb_pool = Arc::new(surrealdb_pool().await.expect("Surrealdb connection error!"));
        let postgresql_pool = Arc::new(postgresql_pool().await);

        // ==================== Surrealdb Repository ==================== //
        //  ADMIN  //
        let admin_repository: Arc<dyn AdminTrait> = Arc::new(AdminSurrealRepository::new(Arc::clone(&surrealdb_pool)));

        //  MANAGEMENT  //
        let user_repository: Arc<dyn UserTrait> = Arc::new(UserSurrealRepository::new(Arc::clone(&surrealdb_pool)));

        let relation_repository: Arc<dyn RelationTrait> = Arc::new(RelationSurrealRepository::new(Arc::clone(&surrealdb_pool)));

        //  ORGANIZATION  //
        let organization_repository: Arc<dyn OrganizationTrait> =
            Arc::new(OrganizationSurrealRepository::new(Arc::clone(&surrealdb_pool)));
        let branch_repository: Arc<dyn BranchTrait> = Arc::new(BranchSurrealRepository::new(Arc::clone(&surrealdb_pool)));

        //  MESSAGE  //
        let telegram_group_repository: Arc<dyn TelegramGroupTrait> =
            Arc::new(TelegramGroupSurrealRepository::new(Arc::clone(&surrealdb_pool)));
        let fcm_subscription_repository: Arc<dyn FCMSubscriptionTrait> =
            Arc::new(FCMSubscriptionSurrealRepository::new(Arc::clone(&surrealdb_pool)));
        let subscription_repository: Arc<dyn SubscriptionTrait> =
            Arc::new(SubscriptionSurrealRepository::new(Arc::clone(&surrealdb_pool)));

        //  COMMON  //
        let common_repository: Arc<dyn CommonRepository> = Arc::new(CommonSurrealRepository::new(Arc::clone(&surrealdb_pool)));

        // ==================== Postgresql Repository ==================== //
        //  ADMIN  //
        let admin_postgresql_repository: Arc<dyn AdminTrait> = Arc::new(AdminPostgresqlRepository::new(Arc::clone(&postgresql_pool)));

        // //  MANAGEMENT  //
        // let user_repository: Arc<dyn UserTrait> = Arc::new(UserPostgresqlRepository::new(Arc::clone(&postgresql_pool)));

        // let relation_repository: Arc<dyn RelationTrait> = Arc::new(RelationPostgresqlRepository::new(Arc::clone(&postgresql_pool)));

        // //  ORGANIZATION  //
        // let organization_repository: Arc<dyn OrganizationTrait> =
        //     Arc::new(OrganizationPostgresqlRepository::new(Arc::clone(&postgresql_pool)));
        // let branch_repository: Arc<dyn BranchTrait> = Arc::new(BranchPostgresqlRepository::new(Arc::clone(&postgresql_pool)));

        // //  MESSAGE  //
        // let telegram_group_repository: Arc<dyn TelegramGroupTrait> =
        //     Arc::new(TelegramGroupPostgresqlRepository::new(Arc::clone(&postgresql_pool)));
        // let fcm_subscription_repository: Arc<dyn FCMSubscriptionTrait> =
        //     Arc::new(FCMSubscriptionPostgresqlRepository::new(Arc::clone(&postgresql_pool)));
        // let subscription_repository: Arc<dyn SubscriptionTrait> =
        //     Arc::new(SubscriptionPostgresqlRepository::new(Arc::clone(&postgresql_pool)));

        // //  COMMON  //
        // let common_repository: Arc<dyn CommonRepository> = Arc::new(CommonPostgresqlRepository::new(Arc::clone(&postgresql_pool)));

        // ==================== Service ==================== //

        let admin_service_surrealdb = Arc::new(AdminServiceImpl {
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

        // ==================== Postgresql Service ==================== //

        let admin_service_postgresql = Arc::new(AdminServiceImpl {
            repository: admin_postgresql_repository.clone(),
        });

        Container {
            admin_service_surrealdb,
            admin_service_postgresql,
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
