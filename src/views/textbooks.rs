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
    models::textbook::{NewTextBook, TextBook},
    HistoryError, HistoryState, HtmlTemplate, IMG_PATH,
};

#[derive(TryFromMultipart)]
pub struct TextBookWithImage {
    pub name: String,
    pub link: String,
    pub description: String,
    pub cover: FieldData<Bytes>,
}

#[derive(Template)]
#[template(path = "admin/textbooks/list.html")]
pub struct TextBooksTemplate {
    pub textbooks: Vec<TextBook>,
}

#[derive(Template)]
#[template(path = "admin/textbooks/add.html")]
pub struct NewTextBookTemplate;

#[derive(Template)]
#[template(path = "admin/textbooks/edit.html")]
pub struct EditTextBookTemplate {
    pub textbook: TextBook,
}

pub async fn all(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let textbooks = TextBook::list(&state.db).await?;
    Ok(HtmlTemplate(TextBooksTemplate { textbooks }))
}

pub async fn add() -> impl IntoResponse {
    HtmlTemplate(NewTextBookTemplate {})
}

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let textbook = TextBook::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EditTextBookTemplate { textbook }))
}

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(textbook_with_image): TypedMultipart<TextBookWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let new_textbook = NewTextBook {
        name: textbook_with_image.name,
        link: textbook_with_image.link,
        description: textbook_with_image.description,
        cover: textbook_with_image
            .cover
            .metadata
            .file_name
            .unwrap_or(String::new()),
    };
    let mut file = File::create(format!("{}/{}", IMG_PATH, new_textbook.cover))?;
    file.write_all(&textbook_with_image.cover.contents)?;
    TextBook::create(&state.db, new_textbook).await?;
    Ok(Redirect::to("/textbooks"))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(textbook_with_image): TypedMultipart<TextBookWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let old_textbook = TextBook::fetch(&state.db, id).await?;
    let file_name = textbook_with_image.cover.metadata.file_name.unwrap();
    let new_cover = if file_name.eq(&"".to_string()) {
        old_textbook.cover
    } else {
        let mut file = File::create(format!("{}/{}", IMG_PATH, file_name))?;
        file.write_all(&textbook_with_image.cover.contents)?;
        remove_file(format!("{}/{}", IMG_PATH, old_textbook.cover))?;
        file_name
    };
    let updated_textbook = NewTextBook {
        name: textbook_with_image.name,
        link: textbook_with_image.link,
        description: textbook_with_image.description,
        cover: new_cover,
    };
    TextBook::update(&state.db, id, updated_textbook).await?;
    Ok(Redirect::to("/textbooks"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let textbook = TextBook::fetch(&state.db, id).await?;
    TextBook::delete(&state.db, id).await?;
    remove_file(format!("{}/{}", IMG_PATH, textbook.cover))?;
    Ok(Redirect::to("/textbooks"))
}
