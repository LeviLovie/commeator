use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    Auth(anyhow::Error),
    Internal(anyhow::Error),
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(err: E) -> Self {
        AppError::Internal(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Auth(e) => {
                eprintln!("Auth error: {e:?}");
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
            AppError::Internal(e) => {
                eprintln!("Internal error: {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}
