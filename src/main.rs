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

#[tokio::main]
async fn main() {
    let secret = env::var("SECRET").expect("SECRET must be set!");
    let state = Arc::new(HistoryState { secret });
    let history = Router::new()
        .route("/", get(home))
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
    let name = "denis".to_string();
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

async fn nothing() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}

#[derive(Template)]
#[template(path = "home.html")]
struct HelloTemplate {
    name: String,
}

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
