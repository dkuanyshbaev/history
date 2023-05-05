use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{env, net::SocketAddr, sync::Arc};

struct HistoryState {
    secret: String,
}

struct Book {
    name: String,
}

struct Post {
    title: String,
}

#[tokio::main]
async fn main() {
    let secret = env::var("SECRET").expect("SECRET must be set!");
    let state = Arc::new(HistoryState { secret });
    let history = Router::new()
        .route("/", get(home))
        .route("/lib", get(lib))
        .route("/blog", get(blog))
        .route("/about", get(about))
        .with_state(state)
        .fallback(nothing);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(history.into_make_service())
        .await
        .unwrap();
}

async fn home() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

async fn lib() -> impl IntoResponse {
    // научные книги и статьи
    // учебники и пособия
    // публицистика
    // проза и поэзия

    let books = vec![
        Book {
            name: "book one".to_string(),
        },
        Book {
            name: "book two".to_string(),
        },
    ];
    HtmlTemplate(LibTemplate { books })
}

async fn blog() -> impl IntoResponse {
    let posts = vec![
        Post {
            title: "post one".to_string(),
        },
        Post {
            title: "post two".to_string(),
        },
    ];
    HtmlTemplate(BlogTemplate { posts })
}

async fn about() -> impl IntoResponse {
    HtmlTemplate(AboutTemplate {})
}

async fn nothing() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "lib.html")]
struct LibTemplate {
    books: Vec<Book>,
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate;

struct HtmlTemplate<T>(T);
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
