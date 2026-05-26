use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub status: i32,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            status: 200,
            message: "success".to_string(),
            data,
        }
    }

    pub fn ok_with_message(message: impl Into<String>, data: T) -> Self {
        Self {
            status: 200,
            message: message.into(),
            data,
        }
    }

    pub fn fail(status: i32, message: impl Into<String>, data: T) -> Self {
        Self {
            status,
            message: message.into(),
            data,
        }
    }
}

impl<T: Serialize + Send> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
