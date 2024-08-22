use async_trait::async_trait;
use chrono::{SubsecRound, Utc};
use common::enums::RelationType;
use common::enums::UserRoleType;
use domain::models::management::CreateRelation;
use domain::repositories::management::{RelationTrait, UserTrait};
use domain::repositories::organization::OrganizationQueryParams;
use std::sync::Arc;
use validator::Validate;

use common::errors::BasicError;
use common::responses::DeleteResponseResult;
use domain::dto::organization::{PatchBranchDTO, PatchOrganizationDTO, PostBranchDTO, PostOrganizationDTO};
use domain::models::organization::{
    Branch, CreateBranch, CreateOrganization, Organization, PatchBranch, PatchOrganization,
};
use domain::repositories::organization::{BranchQueryParams, BranchTrait, OrganizationTrait};
use domain::repositories::repository::ResultPaging;
use domain::services::organization::{BranchService, OrganizationService};

use crate::permission::organization::{
    branch_create_permission_controller, branch_delete_permission_controller, branch_patch_permission_controller,
    organization_delete_patch_permission_controller,
};

// ==================== ORGANIZATION ==================== //
pub struct OrganizationServiceImpl {
    pub repository: Arc<dyn OrganizationTrait>,
    pub branch_repository: Arc<dyn BranchTrait>,
    pub user_repository: Arc<dyn UserTrait>,
    pub relation_repository: Arc<dyn RelationTrait>,
}

impl OrganizationServiceImpl {
    pub fn new(
        repository: Arc<dyn OrganizationTrait>,
        branch_repository: Arc<dyn BranchTrait>,
        user_repository: Arc<dyn UserTrait>,
        relation_repository: Arc<dyn RelationTrait>,
    ) -> Self {
        OrganizationServiceImpl {
            repository,
            branch_repository,
            user_repository,
            relation_repository,
        }
    }
}

#[async_trait]
impl OrganizationService for OrganizationServiceImpl {
    async fn create(&self, data: PostOrganizationDTO, user_id: String) -> Result<Organization, BasicError> {
        self.user_repository.get_user_by_id(&user_id).await?;

        match data.validate() {
            Ok(_) => {
                let organization = self
                    .repository
                    .create(CreateOrganization {
                        name: data.name.clone(),
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await?;

                let branch_name: String = format!("{}_main", data.name.as_str());

                let branch = self
                    .branch_repository
                    .create(CreateBranch {
                        name: branch_name.clone(),
                        organization_id: organization.id.clone(),
                        created_at: Utc::now().trunc_subsecs(0),
                        ..Default::default()
                    })
                    .await?;

                self.relation_repository
                    .create(CreateRelation {
                        organization_id: organization.id.clone(),
                        branch_id: branch.id,
                        user_id,
                        role: UserRoleType::OrganizationOwner,
                        relation_type: RelationType::Relation,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await?;

                Ok(organization)
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn delete(&self, current_id: String, user_id: String) -> Result<DeleteResponseResult, BasicError> {
        organization_delete_patch_permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &current_id,
        )
        .await?;

        self.repository.delete(current_id).await
    }

    async fn get(&self, current_id: String, user_id: String) -> Result<Organization, BasicError> {
        self.user_repository.get_user_by_id(&user_id).await?;

        self.repository.get(current_id).await
    }

    async fn list(
        &self,
        query_params: OrganizationQueryParams,
        user_id: String,
    ) -> Result<ResultPaging<Organization>, BasicError> {
        self.user_repository.get_user_by_id(&user_id).await?;

        self.repository.list(query_params).await
    }

    async fn patch(
        &self,
        current_id: String,
        data: PatchOrganizationDTO,
        user_id: String,
    ) -> Result<Organization, BasicError> {
        organization_delete_patch_permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &current_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .patch(
                        current_id,
                        PatchOrganization {
                            name: data.name,
                            updated_at: Utc::now().trunc_subsecs(0),
                        },
                    )
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }
}

// ==================== BRANCH ==================== //
pub struct BranchServiceImpl {
    pub repository: Arc<dyn BranchTrait>,
    pub user_repository: Arc<dyn UserTrait>,
    pub relation_repository: Arc<dyn RelationTrait>,
}

impl BranchServiceImpl {
    pub fn new(
        repository: Arc<dyn BranchTrait>,
        user_repository: Arc<dyn UserTrait>,
        relation_repository: Arc<dyn RelationTrait>,
    ) -> Self {
        BranchServiceImpl {
            repository,
            user_repository,
            relation_repository,
        }
    }
}

#[async_trait]
impl BranchService for BranchServiceImpl {
    async fn create(
        &self,
        data: PostBranchDTO,
        user_id: String,
        organization_id: String,
    ) -> Result<Branch, BasicError> {
        branch_create_permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                let branch = self
                    .repository
                    .create(CreateBranch {
                        name: data.name,
                        branch_location: data.branch_location,
                        for_call: data.for_call,
                        organization_id: organization_id.clone(),
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await?;

                self.relation_repository
                    .create(CreateRelation {
                        user_id,
                        branch_id: branch.id.clone(),
                        role: UserRoleType::OrganizationOwner,
                        organization_id,
                        relation_type: RelationType::Relation,
                        created_at: Utc::now().trunc_subsecs(0),
                    })
                    .await?;

                Ok(branch)
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }

    async fn delete(
        &self,
        current_id: String,
        user_id: String,
        organization_id: String,
    ) -> Result<DeleteResponseResult, BasicError> {
        branch_delete_permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &current_id,
        )
        .await?;

        self.repository.delete(current_id).await
    }

    async fn get(&self, current_id: String, user_id: String) -> Result<Branch, BasicError> {
        self.user_repository.get_user_by_id(&user_id).await?;

        self.repository.get(current_id).await
    }

    async fn list(&self, query_params: BranchQueryParams, user_id: String) -> Result<ResultPaging<Branch>, BasicError> {
        self.user_repository.get_user_by_id(&user_id).await?;

        self.repository.list(query_params).await
    }

    async fn patch(
        &self,
        current_id: String,
        data: PatchBranchDTO,
        user_id: String,
        organization_id: String,
    ) -> Result<Branch, BasicError> {
        branch_patch_permission_controller(
            self.relation_repository.get_relations_by_user_id(&user_id).await?,
            &organization_id,
            &current_id,
        )
        .await?;

        match data.validate() {
            Ok(_) => {
                self.repository
                    .patch(
                        current_id,
                        PatchBranch {
                            name: data.name,
                            branch_location: data.branch_location,
                            for_call: data.for_call,
                            updated_at: Utc::now().trunc_subsecs(0),
                        },
                    )
                    .await
            }
            Err(e) => Err(BasicError::from(e)),
        }
    }
}
