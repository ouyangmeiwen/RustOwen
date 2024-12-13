use serde::Serialize;
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String, // "success", "error", "fail"
    pub message: Option<String>,
    pub data: Option<T>,
    pub count: Option<i64>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: "success".to_string(),
            message: None,
            data: Some(data),
            count: None,
        }
    }
    pub fn success_without_data() -> Self {
        ApiResponse {
            status: "success".to_string(),
            message: None,
            data: None,
            count: None,
        }
    }
    pub fn success_with_count(data: T, cnt: i64) -> Self {
        ApiResponse {
            status: "success".to_string(),
            message: None,
            data: Some(data),
            count: Some(cnt),
        }
    }
    pub fn success_with_msg(msg: String) -> Self {
        ApiResponse {
            status: "success".to_string(),
            message: Some(msg),
            data: None,
            count: None,
        }
    }
    pub fn error(message: &str) -> Self {
        ApiResponse {
            status: "error".to_string(),
            message: Some(message.to_string()),
            data: None,
            count: None,
        }
    }

    pub fn fail(message: &str) -> Self {
        ApiResponse {
            status: "fail".to_string(),
            message: Some(message.to_string()),
            data: None,
            count: None,
        }
    }
}
