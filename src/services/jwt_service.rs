use chrono::{TimeZone, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::sea_query::Expr;
use sea_orm::{entity::prelude::*, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tonic::metadata::MetadataMap;
use tonic::Status;

use crate::entities::regular_token;

#[derive(Debug, Default)]
pub struct JWTService {
    db: DatabaseConnection,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: i64,
}

impl JWTService {
    pub fn new(db: DatabaseConnection) -> JWTService {
        JWTService { db }
    }

    pub async fn get_database_connection(&self) -> Result<DatabaseConnection, Status> {
        Ok(self.db.clone())
    }
}

pub async fn is_token_valid(metadata: &MetadataMap, jwt_service: &JWTService) -> bool {
    match jwt_service.get_database_connection().await {
        Ok(db) => {
            let token_value = metadata
                .get("authorization")
                .and_then(|value| value.to_str().ok())
                .unwrap_or("");

            match regular_token::Entity::find()
                .filter(regular_token::Column::Token.eq(token_value))
                .one(&db)
                .await
            {
                Ok(Some(token)) => {
                    if token.active && !if_token_expired(token, jwt_service).await {
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

    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    encode(&Header::default(), &claims, &encoding_key).unwrap()
}

async fn if_token_expired(
    token: crate::entities::regular_token::Model,
    jwt_service: &JWTService,
) -> bool {
    let secret = "secret"; // Klucz do dekodowania JWT

    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();

    match decode::<Claims>(&token.token, &decoding_key, &validation) {
        Ok(token_data) => {
            let exp = token_data.claims.exp;
            let expiration_time = Utc.timestamp(exp, 0);
            let current_time = Utc::now();

            if expiration_time < current_time {
                match jwt_service.get_database_connection().await {
                    Ok(_db) => {
                        let _ = make_token_expired(token, jwt_service).await;
                    }
                    Err(_) => {} // Błąd dekodowania oznacza, że token jest nieprawidłowy lub przeterminowany
                }
                true
            } else {
                false
            }
        }
        Err(_) => true, // Błąd dekodowania oznacza, że token jest nieprawidłowy lub przeterminowany
    }
}
async fn make_token_expired(
    token: crate::entities::regular_token::Model,
    jwt_service: &JWTService,
) -> Result<(), Box<dyn std::error::Error>> {
    match jwt_service.get_database_connection().await {
        Ok(db) => {
            regular_token::Entity::update_many()
                .filter(regular_token::Column::Token.eq(token.token.clone()))
                .col_expr(regular_token::Column::Active, Expr::value(false))
                .col_expr(
                    regular_token::Column::ExpirationTime,
                    Expr::value(Utc::now()),
                )
                .exec(&db)
                .await?;
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
