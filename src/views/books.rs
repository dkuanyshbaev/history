use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Form,
};
use axum_typed_multipart::TypedMultipart;
use std::{fs::File, io::prelude::*, sync::Arc};

use crate::{
    models::{Book, BookWithImage, NewBook},
    templates::*,
    HistoryError, HistoryState,
};

pub async fn all(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let books = Book::list(&state.db).await?;
    Ok(HtmlTemplate(BooksTemplate { books }))
}

pub async fn add() -> impl IntoResponse {
    HtmlTemplate(NewBookTemplate {})
}

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let book = Book::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EditBookTemplate { book }))
}

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(book_with_image): TypedMultipart<BookWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let new_book = NewBook {
        name: book_with_image.name,
        link: book_with_image.link,
        description: book_with_image.description,
        cover: book_with_image
            .cover
            .metadata
            .file_name
            .unwrap_or(String::new()),
    };
    let mut file = File::create(format!("static/img/{}", new_book.cover))?;
    file.write_all(&book_with_image.cover.contents)?;
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
