use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String, // "success", "error", "fail"
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: "success".to_string(),
            message: None,
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        ApiResponse {
            status: "error".to_string(),
            message: Some(message.to_string()),
            data: None,
        }
    }

    pub fn fail(message: &str) -> Self {
        ApiResponse {
            status: "fail".to_string(),
            message: Some(message.to_string()),
            data: None,
        }
    }
    // 处理没有数据返回的情况
    pub fn success_no_contend() -> Self {
        ApiResponse {
            status: "success".to_string(),
            message: None,
            data: None,
        }
    }
}
