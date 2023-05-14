use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum HistoryError {
    NotFound,
    InternalError,
}

impl IntoResponse for HistoryError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            HistoryError::NotFound => (StatusCode::NOT_FOUND, "Nothing to see here"),
            HistoryError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error"),
        };
        (status, error_message).into_response()
    }
}

impl From<sqlx::Error> for HistoryError {
    fn from(_: sqlx::Error) -> Self {
        HistoryError::InternalError
    }
}
