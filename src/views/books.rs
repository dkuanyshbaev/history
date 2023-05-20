use askama::Template;
use axum::{
    body::Bytes,
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use std::{
    fs::{remove_file, File},
    io::prelude::*,
    sync::Arc,
};

use crate::{
    models::book::{Book, NewBook},
    HistoryError, HistoryState, HtmlTemplate,
};

const IMG_PATH: &str = "static/img";

#[derive(TryFromMultipart)]
pub struct BookWithImage {
    pub name: String,
    pub link: String,
    pub description: String,
    pub cover: FieldData<Bytes>,
}

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
    let mut file = File::create(format!("{}/{}", IMG_PATH, new_book.cover))?;
    file.write_all(&book_with_image.cover.contents)?;
    Book::create(&state.db, new_book).await?;
    Ok(Redirect::to("/books"))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(book_with_image): TypedMultipart<BookWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let old_book = Book::fetch(&state.db, id).await?;
    let file_name = book_with_image.cover.metadata.file_name.unwrap();
    let new_cover = if file_name.eq(&"".to_string()) {
        old_book.cover
    } else {
        let mut file = File::create(format!("{}/{}", IMG_PATH, file_name))?;
        file.write_all(&book_with_image.cover.contents)?;
        remove_file(format!("{}/{}", IMG_PATH, old_book.cover))?;
        file_name
    };
    let updated_book = NewBook {
        name: book_with_image.name,
        link: book_with_image.link,
        description: book_with_image.description,
        cover: new_cover,
    };
    Book::update(&state.db, id, updated_book).await?;
    Ok(Redirect::to("/books"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let book = Book::fetch(&state.db, id).await?;
    Book::delete(&state.db, id).await?;
    remove_file(format!("{}/{}", IMG_PATH, book.cover))?;
    Ok(Redirect::to("/books"))
}
