use crate::models::claims_model::Claims;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

/// 创建 JWT
pub fn create_jwt(my_claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
