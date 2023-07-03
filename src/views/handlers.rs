use askama::Template;
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::{Book, HistoryError, HistoryState, HtmlTemplate, Publication, Text};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub books: Vec<Book>,
    pub publications: Vec<Publication>,
    pub texts: Vec<Text>,
}

pub async fn home(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let books = Book::list(&state.db).await?;
    let publications = Publication::list(&state.db).await?;
    let texts = Text::list(&state.db).await?;
    Ok(HtmlTemplate(HomeTemplate {
        books,
        publications,
        texts,
    }))
}
