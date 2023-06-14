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
    models::text::{NewText, Text},
    HistoryError, HistoryState, HtmlTemplate, IMG_PATH,
};

#[derive(TryFromMultipart)]
pub struct TextWithImage {
    pub name: String,
    pub link: String,
    pub description: String,
    pub cover: FieldData<Bytes>,
}

#[derive(Template)]
#[template(path = "admin/texts/list.html")]
pub struct TextsTemplate {
    pub texts: Vec<Text>,
}

#[derive(Template)]
#[template(path = "admin/texts/add.html")]
pub struct NewTextTemplate;

#[derive(Template)]
#[template(path = "admin/texts/edit.html")]
pub struct EditTextTemplate {
    pub text: Text,
}

pub async fn all(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let texts = Text::list(&state.db).await?;
    Ok(HtmlTemplate(TextsTemplate { texts }))
}

pub async fn add() -> impl IntoResponse {
    HtmlTemplate(NewTextTemplate {})
}

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let text = Text::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EditTextTemplate { text }))
}

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(text_with_image): TypedMultipart<TextWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let new_text = NewText {
        name: text_with_image.name,
        link: text_with_image.link,
        description: text_with_image.description,
        cover: text_with_image
            .cover
            .metadata
            .file_name
            .unwrap_or(String::new()),
    };
    let mut file = File::create(format!("{}/{}", IMG_PATH, new_text.cover))?;
    file.write_all(&text_with_image.cover.contents)?;
    Text::create(&state.db, new_text).await?;
    Ok(Redirect::to("/texts"))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(text_with_image): TypedMultipart<TextWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let old_text = Text::fetch(&state.db, id).await?;
    let file_name = text_with_image.cover.metadata.file_name.unwrap();
    let new_cover = if file_name.eq(&"".to_string()) {
        old_text.cover
    } else {
        let mut file = File::create(format!("{}/{}", IMG_PATH, file_name))?;
        file.write_all(&text_with_image.cover.contents)?;
        remove_file(format!("{}/{}", IMG_PATH, old_text.cover))?;
        file_name
    };
    let updated_text = NewText {
        name: text_with_image.name,
        link: text_with_image.link,
        description: text_with_image.description,
        cover: new_cover,
    };
    Text::update(&state.db, id, updated_text).await?;
    Ok(Redirect::to("/texts"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let text = Text::fetch(&state.db, id).await?;
    Text::delete(&state.db, id).await?;
    remove_file(format!("{}/{}", IMG_PATH, text.cover))?;
    Ok(Redirect::to("/texts"))
}
