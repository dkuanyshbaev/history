use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Form, Router};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    extractors::AuthContext,
    memory_store::MemoryStore as AuthMemoryStore,
    AuthLayer, AuthUser, RequireAuthorizationLayer,
};
use rand::Rng;
use serde::Deserialize;
use std::{collections::HashMap, env, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

use auth::{Role, User};
use templates::*;

pub mod auth;
pub mod templates;

type Auth = AuthContext<usize, User, AuthMemoryStore<usize, User>, Role>;
type RequireAuth = RequireAuthorizationLayer<usize, User, Role>;

#[derive(Deserialize, Debug)]
struct Input {
    secret: String,
}

pub struct Book {
    name: String,
    description: String,
    cover: String,
}

pub struct Post {
    title: String,
    body: String,
}

#[tokio::main]
async fn main() {
    let secret = env::var("SECRET").expect("SECRET must be set!");
    let session_secret = rand::thread_rng().gen::<[u8; 64]>();
    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &session_secret);

    let store = Arc::new(RwLock::new(HashMap::default()));
    let user = User::new_admin(secret);
    store.write().await.insert(user.get_id(), user);
    let user_store = AuthMemoryStore::new(&store);
    let auth_layer = AuthLayer::new(user_store, &session_secret);

    let history = Router::new()
        .route("/books", get(books))
        .route("/posts", get(posts))
        .route_layer(RequireAuth::login_with_role(Role::Admin..))
        .nest_service("/static", ServeDir::new("static"))
        .route("/login", get(login_form).post(login))
        .route("/logout", get(logout))
        .route("/", get(home))
        .route("/lib", get(lib))
        .route("/blog", get(blog))
        .layer(auth_layer)
        .layer(session_layer)
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
            description: "description one".to_string(),
            cover: "cover one".to_string(),
        },
        Book {
            name: "book two".to_string(),
            description: "description two".to_string(),
            cover: "cover two".to_string(),
        },
    ];
    HtmlTemplate(LibTemplate { books })
}

async fn blog() -> impl IntoResponse {
    let posts = vec![
        Post {
            title: "post one".to_string(),
            body: "body one".to_string(),
        },
        Post {
            title: "post two".to_string(),
            body: "body two".to_string(),
        },
    ];
    HtmlTemplate(BlogTemplate { posts })
}

async fn books(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Books. Logged in as: {}", user.name)
}

async fn posts(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Posts. Logged in as: {}", user.name)
}

async fn login_form() -> impl IntoResponse {
    HtmlTemplate(LoginTemplate {})
}

async fn login(mut auth: Auth, Form(input): Form<Input>) {
    let user = User::new_admin(input.secret);
    auth.login(&user).await.unwrap();
}

async fn logout(mut auth: Auth) {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;
}

async fn nothing() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}
