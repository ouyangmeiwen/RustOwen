use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Claims {
    pub iss: String,      // 发行者
    pub sub: String,      // 主题
    pub aud: String,      // 受众
    pub exp: usize,       // 过期时间
    pub iat: usize,       // 签发时间
    pub nbf: usize,       // 生效时间
    pub jti: String,      // 唯一 ID
    pub user_id: String,  // 用户 ID
    pub role: String,     // 用户角色
    pub username: String, // 用户名
}

/// 请求体结构
#[derive(Deserialize)]
pub struct TokenRequest {
    pub user_id: String, // 用户 ID 或邮箱
}

/// 响应结构
#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}
