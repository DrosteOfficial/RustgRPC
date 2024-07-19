use sea_orm::entity::prelude::*;
use crate::entities::user;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(skip_deserializing)]
    pub id: i32,
    pub message: String,
    pub sender: i32,
    pub receiver: i32,
    pub timestamp: i64,
}
#[derive(Copy, Clone, Debug, EnumIter,DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::Sender",
        to = "user::Column::Id"
    )]
    User,
}



impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}

