use askama::Template;
use axum::{
    body::Bytes,
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use chrono::Local;
use std::{
    fs::{remove_file, File},
    io::prelude::*,
    sync::Arc,
};

use crate::{
    models::post::{NewPost, Post},
    HistoryError, HistoryState, HtmlTemplate, IMG_PATH,
};

#[derive(TryFromMultipart)]
pub struct PostWithImage {
    pub title: String,
    pub lead: String,
    pub body: String,
    pub cover: FieldData<Bytes>,
}

#[derive(Template)]
#[template(path = "admin/posts/list.html")]
pub struct PostsTemplate {
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "admin/posts/add.html")]
pub struct NewPostTemplate;

#[derive(Template)]
#[template(path = "admin/posts/edit.html")]
pub struct EditPostTemplate {
    pub post: Post,
}

pub async fn all(
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let posts = Post::list(&state.db).await?;
    Ok(HtmlTemplate(PostsTemplate { posts }))
}

pub async fn add() -> impl IntoResponse {
    HtmlTemplate(NewPostTemplate {})
}

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let post = Post::fetch(&state.db, id).await?;
    Ok(HtmlTemplate(EditPostTemplate { post }))
}

pub async fn create(
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(post_with_image): TypedMultipart<PostWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let file_name = Local::now().timestamp().to_string()
        + "_"
        + &post_with_image
            .cover
            .metadata
            .file_name
            .unwrap_or(String::new());
    let new_post = NewPost {
        title: post_with_image.title,
        lead: post_with_image.lead,
        body: post_with_image.body,
        cover: file_name,
    };
    let mut file = File::create(format!("{}/{}", IMG_PATH, new_post.cover))?;
    file.write_all(&post_with_image.cover.contents)?;
    Post::create(&state.db, new_post).await?;
    Ok(Redirect::to("/posts"))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
    TypedMultipart(post_with_image): TypedMultipart<PostWithImage>,
) -> Result<impl IntoResponse, HistoryError> {
    let old_post = Post::fetch(&state.db, id).await?;
    let file_name = Local::now().timestamp().to_string()
        + "_"
        + &post_with_image
            .cover
            .metadata
            .file_name
            .unwrap_or(String::new());
    let new_cover = if file_name.eq(&"".to_string()) {
        old_post.cover
    } else {
        let mut file = File::create(format!("{}/{}", IMG_PATH, file_name))?;
        file.write_all(&post_with_image.cover.contents)?;
        remove_file(format!("{}/{}", IMG_PATH, old_post.cover))?;
        file_name
    };
    let updated_post = NewPost {
        title: post_with_image.title,
        lead: post_with_image.lead,
        body: post_with_image.body,
        cover: new_cover,
    };
    Post::update(&state.db, id, updated_post).await?;
    Ok(Redirect::to("/posts"))
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<HistoryState>>,
) -> Result<impl IntoResponse, HistoryError> {
    let post = Post::fetch(&state.db, id).await?;
    Post::delete(&state.db, id).await?;
    remove_file(format!("{}/{}", IMG_PATH, post.cover))?;
    Ok(Redirect::to("/posts"))
}
