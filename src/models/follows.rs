use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "follows")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub follower_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub following_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::FollowerId",
        to = "super::users::Column::Id"
    )]
    Follower,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::FollowingId",
        to = "super::users::Column::Id"
    )]
    Following,
}

impl ActiveModelBehavior for ActiveModel {}