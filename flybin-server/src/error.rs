use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError(pub StatusCode, pub String);
impl AppError {
    pub fn internal() -> Self {
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error occured".to_string(),
        )
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self(StatusCode::NOT_FOUND, "Paste not found \n".to_string()),
            _ => Self::internal(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}
