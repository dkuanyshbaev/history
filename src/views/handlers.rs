use askama::Template;
use axum::response::IntoResponse;

use crate::{Book, HtmlTemplate};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "lib.html")]
pub struct LibTemplate {
    pub books: Vec<Book>,
}

pub async fn home() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

pub async fn lib() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

pub async fn blog() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}
