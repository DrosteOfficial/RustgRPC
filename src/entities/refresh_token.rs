// use crate::entities::user;
// use sea_orm::entity::prelude::*;
//
// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
// #[sea_orm(table_name = "RefreshToken")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub token: String,
//     pub active: bool,
//     pub creaitontime: DateTime,
//     pub expired: bool,
//     pub expirationtime: DateTime,
//     pub receiver: i32,
//     pub timestamp: i64,
// }
// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(
//         belongs_to = "user::Entity",
//         from = "Column::id",
//         to = "user::Column::Id"
//     )]
//     User,
// }
//
// impl Related<user::Entity> for crate::entities::refresh_token::Entity {
//     fn to() -> RelationDef {
//         crate::entities::message::Relation::User.def()
//     }
//     fn via() -> Option<RelationDef> {
//         Some(Relation::User.def())
//     }
// }
