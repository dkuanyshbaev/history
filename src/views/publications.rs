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
    models::publication::{NewPublication, Publication},
    HistoryError, HistoryState, HtmlTemplate, IMG_PATH,
};

#[derive(TryFromMultipart)]
pub struct PublicationWithImage {
    pub name: String,
    pub link: String,
    pub description: String,
    pub cover: FieldData<Bytes>,
}

#[derive(Template)]
#[template(path = "admin/publications/list.html")]
pub struct PublicationsTemplate {
    pub publications: Vec<Publication>,
}

#[derive(Template)]
#[template(path = "admin/publications/add.html")]
pub struct NewPublicationTemplate;

#[derive(Template)]
#[template(path = "admin/publications/edit.html")]
pub struct EditPublicationTemplate {
    pub publication: Publication,
}

pub async fn all(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let publications = Publication::list(&state.db).await?;
    Ok(HtmlTemplate(PublicationsTemplate { publications }))
}

pub async fn add() -> impl IntoResponse {
    HtmlTemplate(NewPublicationTemplate {})
}

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let publication = Publication::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EditPublicationTemplate { publication }))
}

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(publication_with_image): TypedMultipart<PublicationWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let new_publication = NewPublication {
        name: publication_with_image.name,
        link: publication_with_image.link,
        description: publication_with_image.description,
        cover: publication_with_image
            .cover
            .metadata
            .file_name
            .unwrap_or(String::new()),
    };
    let mut file = File::create(format!("{}/{}", IMG_PATH, new_publication.cover))?;
    file.write_all(&publication_with_image.cover.contents)?;
    Publication::create(&state.db, new_publication).await?;
    Ok(Redirect::to("/publications"))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(publication_with_image): TypedMultipart<PublicationWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let old_publication = Publication::fetch(&state.db, id).await?;
    let file_name = publication_with_image.cover.metadata.file_name.unwrap();
    let new_cover = if file_name.eq(&"".to_string()) {
        old_publication.cover
    } else {
        let mut file = File::create(format!("{}/{}", IMG_PATH, file_name))?;
        file.write_all(&publication_with_image.cover.contents)?;
        remove_file(format!("{}/{}", IMG_PATH, old_publication.cover))?;
        file_name
    };
    let updated_publication = NewPublication {
        name: publication_with_image.name,
        link: publication_with_image.link,
        description: publication_with_image.description,
        cover: new_cover,
    };
    Publication::update(&state.db, id, updated_publication).await?;
    Ok(Redirect::to("/publications"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let publication = Publication::fetch(&state.db, id).await?;
    Publication::delete(&state.db, id).await?;
    remove_file(format!("{}/{}", IMG_PATH, publication.cover))?;
    Ok(Redirect::to("/publications"))
}
