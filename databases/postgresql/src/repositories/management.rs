use std::sync::Arc;

use async_trait::async_trait;
use common::responses::DeleteResponseResult;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::dao::management::{RelationQueryParamsTrait, UserQueryParamsTrait};
use crate::data::management::{
    CreateRelationSurreal, CreateUserSurreal, GetRelationSurreal, GetUserSurreal, PatchInviteToBranchSurreal,
    PatchRelationSurreal, PatchUserSurreal, PutUserPasswordDiesel,
};

use common::errors::BasicError;
use common::types::BasicResult;
use domain::models::management::{
    CreateRelation, CreateUser, PatchInviteToBranch, PatchRelation, PatchUser, PutUserPassword,
};
use domain::models::management::{Relation, User};
use domain::repositories::management::{RelationQueryParams, RelationTrait, UserQueryParams, UserTrait};
use domain::repositories::repository::{ResultPaging, ResultPagingDB};

// ==================== USER ==================== //
pub struct UserSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl UserSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        UserSurrealRepository { pool: db }
    }
}

#[async_trait]
impl UserTrait for UserSurrealRepository {
    async fn get_by_phone_number(&self, phone_number: &str) -> BasicResult<User> {
        let result: Option<GetUserSurreal> = self
            .pool
            .query("SELECT * FROM user WHERE phone_number = $phone_number;")
            .bind(("phone_number", phone_number))
            .await?
            .take(0)?;
        match result {
            Some(user) => Ok(user.into()),
            None => Err(BasicError::not_found_error(String::from(
                "User by current phone number not found!",
            ))),
        }
    }

    async fn get_user_by_id(&self, id: &str) -> BasicResult<User> {
        let result: Option<GetUserSurreal> = self.pool.select(("user", id)).await?;

        match result {
            Some(user) => Ok(user.into()),
            None => Err(BasicError::not_found_error(String::from(
                "User by current id not found!",
            ))),
        }
    }

    //
    async fn change_password(&self, id: String, data: PutUserPassword) -> BasicResult<String> {
        let result: Option<GetUserSurreal> = self
            .pool
            .update(("user", id))
            .merge(PutUserPasswordDiesel::from(data))
            .await?;
        match result {
            Some(_) => Ok(String::from("Password successfully updated!")),
            None => Err(BasicError::not_found_error(String::from("User not found!"))),
        }
    }

    //
    async fn create(&self, user: CreateUser) -> BasicResult<User> {
        let result: Option<GetUserSurreal> = self
            .pool
            .create("user")
            .content(CreateUserSurreal::from(user))
            .await?
            .pop();

        match result {
            Some(user_surreal) => Ok(user_surreal.into()),
            None => Err(BasicError::cannot_create_error(String::from(
                "User cannot be created!",
            ))),
        }
    }

    async fn get(&self, current_id: String) -> BasicResult<User> {
        let result: Option<GetUserSurreal> = self.pool.select(("user", current_id)).await?;

        match result {
            Some(user) => Ok(user.into()),
            None => Err(BasicError::not_found_error(String::from("User not found!"))),
        }
    }

    async fn list(&self, query_params: UserQueryParams) -> BasicResult<ResultPaging<User>> {
        let query_data = query_params.query_params();

        let result: Option<ResultPagingDB<GetUserSurreal>> = self
            .pool
            .query(format!(
                "$limit = {}; $offset = {}; $start = {}; $result = SELECT * FROM user {} START $start LIMIT $limit;\
                RETURN {{total: COUNT(SELECT count() FROM user), limit: $limit, count: COUNT($result), page: $offset, items: $result}};",
                query_data.limit.unwrap_or_default(),
                query_data.offset.unwrap_or_default(),
                query_data.start.unwrap_or_default(),
                query_data.result.unwrap_or_default(),
            ))
            .await?
            .take(4)?;
        result
            .ok_or(BasicError::bad_request_error(String::from("Bad request!")))
            .map(ResultPagingDB::into)
    }

    //
    async fn delete_self(&self, self_id: String) -> BasicResult<DeleteResponseResult> {
        let result: Option<GetUserSurreal> = self.pool.delete(("user", &self_id)).await?;

        self.pool
            .query(
                "DELETE relation WHERE user_id = $self_id;\
                DELETE fcm_subsctiption WHERE user_id = $self_id;\
                DELETE subsctiption WHERE user_id = $self_id;",
            )
            .bind(("self_id", self_id))
            .await?;

        match result.is_some() {
            true => Ok(DeleteResponseResult { status_code: 204 }),
            false => Err(BasicError::not_found_error(String::from("User not found!"))),
        }
    }

    async fn get_self(&self, self_id: String) -> BasicResult<User> {
        let result: Option<GetUserSurreal> = self.pool.select(("user", self_id)).await?;

        match result {
            Some(user) => Ok(user.into()),
            None => Err(BasicError::not_found_error(String::from("User not found!"))),
        }
    }

    async fn patch_self(&self, self_id: String, user: PatchUser) -> BasicResult<User> {
        let result: Option<GetUserSurreal> = self
            .pool
            .update(("user", self_id))
            .merge(PatchUserSurreal::from(user))
            .await?;
        match result {
            Some(user) => Ok(user.into()),
            None => Err(BasicError::not_found_error(String::from("User not found!"))),
        }
    }
}

// ==================== Relation ==================== //
pub struct RelationSurrealRepository {
    pub pool: Arc<Surreal<Client>>,
}

impl RelationSurrealRepository {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        RelationSurrealRepository { pool: db }
    }
}

#[async_trait]
impl RelationTrait for RelationSurrealRepository {
    async fn get_relations_by_user_id(&self, user_id: &str) -> BasicResult<Vec<Relation>> {
        let relations: Vec<GetRelationSurreal> = self
            .pool
            .query(format!(
                "SELECT * FROM relation WHERE user_id = user:{};",
                user_id
            ))
            .await?
            .take(0)?;

        Ok(relations.into_iter().map(|relations| relations.into()).collect())
    }

    //
    async fn delete(&self, current_id: String) -> BasicResult<DeleteResponseResult> {
        let result: Option<GetRelationSurreal> = self.pool.delete(("relation", current_id)).await?;

        match result.is_some() {
            true => Ok(DeleteResponseResult { status_code: 204 }),
            false => Err(BasicError::not_found_error(String::from("User not found!"))),
        }
    }

    async fn list_my_relations(&self, user_id: String) -> BasicResult<Vec<Relation>> {
        let my_relations: Vec<GetRelationSurreal> = self
            .pool
            .query("SELECT * FROM relation WHERE user_id = user:$user_id")
            .bind(("user_id", user_id))
            .await?
            .take(0)?;

        Ok(my_relations.into_iter().map(|result| result.into()).collect())
    }

    //
    async fn create(&self, data: CreateRelation) -> BasicResult<Relation> {
        let result: Option<GetRelationSurreal> = self
            .pool
            .create("relation")
            .content(CreateRelationSurreal::from(data))
            .await?
            .pop();
        match result {
            Some(data) => Ok(data.into()),
            None => Err(BasicError::cannot_create_error(String::from(
                "Relation cannot be created!",
            ))),
        }
    }

    async fn patch_invitation_to_branch(
        &self,
        id: String,
        relation: PatchInviteToBranch,
    ) -> BasicResult<Relation> {
        let result: Option<GetRelationSurreal> = self
            .pool
            .update(("relation", id))
            .merge(PatchInviteToBranchSurreal::from(relation))
            .await?;
        match result {
            Some(relation) => Ok(relation.into()),
            None => Err(BasicError::not_found_error(String::from(
                "InvitationRelation not found!",
            ))),
        }
    }

    async fn list(&self, query_params: RelationQueryParams) -> BasicResult<ResultPaging<Relation>> {
        let query_data = query_params.query_params();
        let result: Option<ResultPagingDB<GetRelationSurreal>> = self
            .pool
            .query(format!(
                "$limit = {}; $offset = {}; $start = {}; $result = SELECT * FROM relation {} START $start LIMIT $limit;\
            RETURN {{total: COUNT(SELECT count() FROM relation), limit: $limit, count: COUNT($result), page: $offset, items: $result}};",
                query_data.limit.unwrap_or_default(),
                query_data.offset.unwrap_or_default(),
                query_data.start.unwrap_or_default(),
                query_data.result.unwrap_or_default(),
            ))
            .await?
            .take(4)?;
        result
            .ok_or(BasicError::bad_request_error(String::from("Bad request!")))
            .map(ResultPagingDB::into)
    }

    async fn patch(&self, id: String, relation: PatchRelation) -> BasicResult<Relation> {
        let result: Option<GetRelationSurreal> = self
            .pool
            .update(("relation", id))
            .merge(PatchRelationSurreal::from(relation))
            .await?;
        match result {
            Some(relation) => Ok(relation.into()),
            None => Err(BasicError::not_found_error(String::from(
                "Relation cannot be created!",
            ))),
        }
    }
}
