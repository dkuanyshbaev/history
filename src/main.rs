use axum::{
    routing::{get, post},
    Router,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    extractors::AuthContext,
    memory_store::MemoryStore as AuthMemoryStore,
    AuthLayer, AuthUser, RequireAuthorizationLayer,
};
use rand::Rng;
use serde::Deserialize;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::{collections::HashMap, env, net::SocketAddr, process, sync::Arc};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

use auth::{Role, User};
use error::HistoryError;
use models::Book;
use views::*;

pub mod auth;
pub mod error;
pub mod models;
pub mod templates;
pub mod views;

const DB_FILE: &str = "db/history.db";

type Auth = AuthContext<usize, User, AuthMemoryStore<usize, User>, Role>;
type RequireAuth = RequireAuthorizationLayer<usize, User, Role>;

#[derive(Deserialize, Debug)]
pub struct LoginInput {
    secret: String,
}

pub struct HistoryState {
    secret: String,
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    let secret = env::var("SECRET").unwrap_or_else(|_| {
        println!("SECRET must be set");
        process::exit(0);
    });
    let db = SqlitePoolOptions::new()
        .connect(DB_FILE)
        .await
        .unwrap_or_else(|_| {
            println!("Can't find db file");
            process::exit(0);
        });
    let state = Arc::new(HistoryState {
        secret: secret.clone(),
        db,
    });

    let session_secret = rand::thread_rng().gen::<[u8; 64]>();
    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &session_secret);

    let store = Arc::new(RwLock::new(HashMap::default()));
    let user = User::new(secret);
    store.write().await.insert(user.get_id(), user);
    let user_store = AuthMemoryStore::new(&store);
    let auth_layer = AuthLayer::new(user_store, &session_secret);

    let history = Router::new()
        // Books
        .route("/books", get(books::all))
        .route("/books/new", get(books::form).post(books::create))
        .route("/books/:id", post(books::update).post(books::delete))
        .route_layer(RequireAuth::login_with_role(Role::Admin..))
        .nest_service("/static", ServeDir::new("static"))
        .route("/login", get(admin::form).post(admin::login))
        .route("/logout", get(admin::logout))
        .route("/", get(handlers::home))
        .route("/lib", get(handlers::lib))
        .route("/blog", get(handlers::blog))
        .fallback(handlers::nothing)
        .layer(auth_layer)
        .layer(session_layer)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(history.into_make_service())
        .await
        .unwrap();
}
