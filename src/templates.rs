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

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "lib.html")]
pub struct LibTemplate {
    pub books: Vec<Book>,
}

// #[derive(Template)]
// #[template(path = "blog.html")]
// pub struct BlogTemplate {
//     pub posts: Vec<Post>,
// }

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path = "admin/books.html")]
pub struct BooksTemplate {
    pub books: Vec<Book>,
}

#[derive(Template)]
#[template(path = "admin/book.html")]
pub struct BookTemplate {
    pub book: Book,
}

#[derive(Template)]
#[template(path = "admin/book_form.html")]
pub struct NewBookTemplate;

// #[derive(Template)]
// #[template(path = "admin/posts.html")]
// pub struct PostsTemplate {
//     pub posts: Vec<Post>,
// }
