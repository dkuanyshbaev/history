use askama::Template;
use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Redirect},
};
use std::sync::Arc;

use crate::{
    models::text::{NewText, Text},
    HistoryError, HistoryState, HtmlTemplate,
};

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

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    Form(new_text): Form<NewText>,
) -> Result<impl IntoResponse, HistoryError> {
    Text::create(&state.db, new_text).await?;
    Ok(Redirect::to("/texts"))
}

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let text = Text::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EditTextTemplate { text }))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    Form(updated_text): Form<NewText>,
) -> Result<impl IntoResponse, HistoryError> {
    Text::update(&state.db, id, updated_text).await?;
    Ok(Redirect::to("/texts"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    Text::delete(&state.db, id).await?;
    Ok(Redirect::to("/texts"))
}
