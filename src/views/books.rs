use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Form,
};
use std::sync::Arc;

use crate::{
    models::{Book, NewBook},
    templates::*,
    HistoryError, HistoryState,
};

pub async fn all(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let books = Book::list(&state.db).await?;
    Ok(HtmlTemplate(BooksTemplate { books }))
}

pub async fn form() -> impl IntoResponse {
    HtmlTemplate(NewBookTemplate {})
}

pub async fn show(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let book = Book::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(BookTemplate { book }))
}

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    Form(new_book): Form<NewBook>,
) -> Result<impl IntoResponse, HistoryError> {
    Book::create(&state.db, new_book).await?;
    Ok(Redirect::to("/books"))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    Form(new_book): Form<NewBook>,
) -> Result<impl IntoResponse, HistoryError> {
    Book::update(&state.db, id, new_book).await?;
    Ok(Redirect::to("/books"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    Book::delete(&state.db, id).await?;
    Ok(Redirect::to("/books"))
}
