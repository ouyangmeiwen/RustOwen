use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use chrono::Utc;

/// JWT Claims
#[derive(Debug, Serialize)]
pub struct Claims {
    pub sub: String, // 用户标识
    pub exp: usize,  // 过期时间
}

/// 创建 JWT
pub fn create_jwt(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let my_claims = Claims {
        sub: user_id,
        exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref()))
}