use chrono::Utc;
use sea_orm::{
    entity::prelude::DateTimeWithTimeZone, ActiveModelTrait, ActiveValue::Set, ColumnTrait,
    DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{
    error::{AppError, Result},
    models::users::{ActiveModel, Column, Entity, Model, UserCDTO},
};

pub struct UserRepository<'db> {
    db: &'db DatabaseConnection,
}

impl<'db> UserRepository<'db> {
    pub fn new(db: &'db DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Model>> {
        Entity::find()
            .all(self.db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn find_by_username(&self, username: String) -> Result<Option<Model>> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(self.db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn create(
        &self,
        UserCDTO {
            username,
            display_name,
        }: UserCDTO,
    ) -> Result<Model> {
        let active = ActiveModel {
            username: Set(username),
            display_name: Set(display_name),
            created_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        };
        active.insert(self.db).await.map_err(AppError::Database)
    }

    
}
