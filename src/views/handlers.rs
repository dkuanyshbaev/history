use askama::Template;
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::{Book, HistoryError, HistoryState, HtmlTemplate};

#[derive(Template)]
#[template(path = "main/home.html")]
pub struct HomeTemplate {
    pub books: Vec<Book>,
}

pub async fn home(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let books = Book::list(&state.db).await?;
    Ok(HtmlTemplate(HomeTemplate { books }))
}
