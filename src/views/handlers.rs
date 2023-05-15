use axum::response::IntoResponse;

use crate::{templates::*, HistoryError};

pub async fn home() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

pub async fn lib() -> impl IntoResponse {
    // научные книги и статьи
    // учебники и пособия
    // публицистика
    // проза и поэзия
    HtmlTemplate(HomeTemplate {})
}

pub async fn blog() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

pub async fn nothing() -> HistoryError {
    HistoryError::NotFound
}
