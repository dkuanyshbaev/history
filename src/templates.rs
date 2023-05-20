use crate::Book;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub struct HtmlTemplate<T>(pub T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

// Handlers:
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "lib.html")]
pub struct LibTemplate {
    pub books: Vec<Book>,
}

// Admin:
#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct LoginTemplate;

// Books:
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
