use sea_orm::entity::prelude::*;
use sea_orm::{TryGetable, TryGetError, Value};
use sea_orm::sea_query::{ArrayType, ValueType, ValueTypeErr};
use serde::{Deserialize, Serialize};

use crate::entities::message;

#[derive(Clone, Debug, Eq, PartialEq, EnumIter, Deserialize, Serialize)]
pub enum GenderTypes {
    Male = 0,
    Female = 1,
    Other = 2,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub gender: GenderTypes,
}

impl From<GenderTypes> for i32 {
    fn from(gender: GenderTypes) -> Self {
        match gender {
            GenderTypes::Male => 0,
            GenderTypes::Female => 1,
            GenderTypes::Other => 2,
        }
    }
}

impl From<i32> for GenderTypes {
    fn from(value: i32) -> Self {
        match value {
            0 => GenderTypes::Male,
            1 => GenderTypes::Female,
            _ => GenderTypes::Other,
        }
    }
}

impl Into<Value> for GenderTypes {
    fn into(self) -> Value {
        let value: i32 = self.into();
        Value::Int(Some(value))
    }
}

impl TryGetable for GenderTypes {
    fn try_get_by<I: sea_orm::ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: i32 = res.try_get_by(index)?;
        Ok(value.into())
    }

    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        let value: i32 = res.try_get(pre, col)?;
        Ok(value.into())
    }
}

impl ValueType for GenderTypes {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Int(Some(i)) => Ok(GenderTypes::from(i)),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "GenderTypes".to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::Int
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        ColumnType::Integer
    }
}

#[derive(Copy, Clone, Debug, EnumIter,DeriveRelation )]
pub enum Relation {
    #[sea_orm(has_many = "message::Entity")]
    Message,
    #[sea_orm(has_many = "message::Entity")]
    RegularToken,
    #[sea_orm(has_many = "message::Entity")]
    RefreshToken,
}


impl Related<message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def();
        Relation::RefreshToken.def();
        Relation::RegularToken.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}
