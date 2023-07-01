use askama::Template;
use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Redirect},
};
use std::sync::Arc;

use crate::{
    models::publication::{NewPublication, Publication},
    HistoryError, HistoryState, HtmlTemplate,
};

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

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    Form(new_publication): Form<NewPublication>,
) -> Result<impl IntoResponse, HistoryError> {
    Publication::create(&state.db, new_publication).await?;
    Ok(Redirect::to("/publications"))
}

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let publication = Publication::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EditPublicationTemplate { publication }))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    Form(updated_publication): Form<NewPublication>,
) -> Result<impl IntoResponse, HistoryError> {
    Publication::update(&state.db, id, updated_publication).await?;
    Ok(Redirect::to("/publications"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    Publication::delete(&state.db, id).await?;
    Ok(Redirect::to("/publications"))
}
