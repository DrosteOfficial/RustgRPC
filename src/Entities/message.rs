use sea_orm::entity::prelude::*;

#[derive( Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub message: String,
    pub sender: i32,
    pub receiver: i32,
    pub timestamp: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {}
impl sea_orm::entity::RelationTrait for Relation {
    fn def(&self) -> sea_orm::entity::RelationDef {
        panic!("No relations are defined for Relation")
    }
}

