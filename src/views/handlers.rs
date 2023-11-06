use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;

use crate::{Book, HistoryError, HistoryState, HtmlTemplate, Post, Publication, Text};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub books: Vec<Book>,
    pub publications: Vec<Publication>,
    pub texts: Vec<Text>,
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "entry.html")]
pub struct EntryTemplate {
    pub post: Post,
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

pub async fn blog(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let posts = Post::list(&state.db).await?;
    Ok(HtmlTemplate(BlogTemplate { posts }))
}

pub async fn entry(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let post = Post::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EntryTemplate { post }))
}
