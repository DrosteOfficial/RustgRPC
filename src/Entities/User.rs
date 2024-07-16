use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, ColumnTrait, EntityTrait, PrimaryKeyTrait, ActiveModelBehavior};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub hashed_password: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl sea_orm::entity::RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations are defined for Relation")
    }
}
impl ActiveModelBehavior for ActiveModel {}