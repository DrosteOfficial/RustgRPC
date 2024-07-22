use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

use crate::entities::user;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "regular_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub active: bool,
    pub creation_time: NaiveDateTime,
    pub expired: bool,
    pub expiration_time: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::UserId",
        to = "user::Column::Id"
    )]
    User,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        None
    }
}

impl ActiveModelBehavior for ActiveModel {}
