use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::{NotSet, Set}, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{
        follows,
        tweets::{self, TweetCDto},
        users,
    },
};

pub struct TweetRepository<'db> {
    db: &'db DatabaseConnection,
}

impl<'db> TweetRepository<'db> {
    pub fn new(db: &'db DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<tweets::Model>> {
        tweets::Entity::find()
            .order_by_desc(tweets::Column::CreatedAt)
            .all(self.db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn get_by_username(&self, username: String) -> Result<Vec<tweets::Model>> {
        let conditions = Condition::any().add(tweets::Column::AuthorUsername.contains(username));
        tweets::Entity::find()
            .filter(conditions)
            .order_by_desc(tweets::Column::CreatedAt)
            .all(self.db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn get_feed(&self, user_id: Uuid) -> Result<Vec<tweets::Model>> {
        tweets::Entity::find()
            .join(JoinType::InnerJoin, tweets::Relation::User.def())
            .join_rev(
                JoinType::InnerJoin,
                follows::Entity::belongs_to(tweets::Entity)
                    .from(follows::Column::FollowingId)
                    .to(tweets::Column::AuthorId)
                    .into(),
            )
            .filter(follows::Column::FollowerId.eq(user_id))
            .order_by_desc(tweets::Column::CreatedAt)
            .all(self.db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn create(
        &self,
        TweetCDto { author_id, content }: TweetCDto,
    ) -> Result<tweets::Model> {
        let author = users::Entity::find_by_id(author_id)
            .one(self.db)
            .await
            .map_err(AppError::Database)?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        let model = tweets::ActiveModel {
            id: NotSet,
            author_id: Set(author_id),
            content: Set(content),
            created_at: Set(Utc::now().fixed_offset()),
            author_username: Set(author.username),
            author_display_name: Set(author.display_name),
            author_avatar_url: Set(author.avatar_url)
        };

        model.insert(self.db).await.map_err(AppError::Database)
    }
}
