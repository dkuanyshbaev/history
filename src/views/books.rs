use askama::Template;
use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Redirect},
};
use std::sync::Arc;

use crate::{
    models::book::{Book, NewBook},
    HistoryError, HistoryState, HtmlTemplate,
};

#[derive(Template)]
#[template(path = "admin/books/list.html")]
pub struct BooksTemplate {
    pub books: Vec<Book>,
}

#[derive(Template)]
#[template(path = "admin/books/add.html")]
pub struct NewBookTemplate;

#[derive(Template)]
#[template(path = "admin/books/edit.html")]
pub struct EditBookTemplate {
    pub book: Book,
}

pub async fn all(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let books = Book::list(&state.db).await?;
    Ok(HtmlTemplate(BooksTemplate { books }))
}

pub async fn add() -> impl IntoResponse {
    HtmlTemplate(NewBookTemplate {})
}

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    Form(new_book): Form<NewBook>,
) -> Result<impl IntoResponse, HistoryError> {
    Book::create(&state.db, new_book).await?;
    Ok(Redirect::to("/books"))
}

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let book = Book::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EditBookTemplate { book }))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    Form(updated_book): Form<NewBook>,
) -> Result<impl IntoResponse, HistoryError> {
    Book::update(&state.db, id, updated_book).await?;
    Ok(Redirect::to("/books"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    Book::delete(&state.db, id).await?;
    Ok(Redirect::to("/books"))
}
