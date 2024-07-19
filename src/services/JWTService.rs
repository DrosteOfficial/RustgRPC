use jsonwebtoken::{encode, Header};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use tonic::metadata::MetadataMap;
use tonic::Status;

use crate::entities::RegularToken;

#[derive(Debug, Default)]
pub struct JWTService {
    DB: DatabaseConnection,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: i64,
}

impl JWTService {
    pub fn new(DB: DatabaseConnection) -> JWTService {
        JWTService { DB }
    }

    pub async fn get_database_connection(&self) -> Result<DatabaseConnection, Status> {
        Ok(self.DB.clone())
    }
}

pub async fn is_token_valid(metadata: &MetadataMap, jwt_service: &JWTService) -> bool {
    match jwt_service.get_database_connection().await {
        Ok(db) => {
            match RegularToken::Entity::find()
                .filter(RegularToken::Column::Token.eq(metadata.clone().into_headers().get("authorization").unwrap().to_str().unwrap()))
                .one(&db).await {
                Ok(Some(token)) => {
                    if token.active && !if_token_expired(token).await {
                        true
                    } else {
                        false
                    }

                }
                Ok(None) => false, // Token not found
                Err(_) => false,   // Database query error
            }
        }
        Err(_) => false, // Failed to get a database connection
    }
}

pub async fn generate_jwt(userclaims: crate::entities::user::Model) -> String {
    let user_str = serde_json::to_string(&userclaims).unwrap();
    let secret = "secret";
    let claims = Claims {
        sub: user_str.to_string(),
        company: "Control".to_string(),
        exp: Utc::now().timestamp() + 60 * 60 * 24,
    };

    encode(&Header::default(), &claims, secret.as_ref()).unwrap()
}

async fn if_token_expired(token: crate::entities::RegularToken::Model,jwt_service: &JWTService ) -> bool {
    if token.expirationtime.timestamp() < Utc::now().timestamp() {
        match JWTService::get_database_connection(jwt_service)
        {
            Ok(db) => {
                let _ = RegularToken::Entity::update_many()
                    .filter(RegularToken::Column::Token.eq(token.token.clone()))
                    .set(RegularToken::Column::Active.eq(false))
                    .set(RegularToken::Column::Expirationtime.eq(Utc::now()))
                    .exec(&db)
                    .await;
            }
            Err(_) => {}
        }
        true
    } else {
        false
    }
}



