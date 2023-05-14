use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    routing::get,
    Form, Router,
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
use templates::*;

pub mod auth;
pub mod error;
pub mod models;
pub mod templates;

const DB_FILE: &str = "db/history.db";

type Auth = AuthContext<usize, User, AuthMemoryStore<usize, User>, Role>;
type RequireAuth = RequireAuthorizationLayer<usize, User, Role>;

#[derive(Deserialize, Debug)]
struct LoginInput {
    secret: String,
}

struct HistoryState {
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
        .route("/books", get(books))
        .route_layer(RequireAuth::login_with_role(Role::Admin..))
        .nest_service("/static", ServeDir::new("static"))
        .route("/login", get(login_form).post(login))
        .route("/logout", get(logout))
        .route("/", get(home))
        // .route("/lib", get(lib))
        // .route("/blog", get(blog))
        .layer(auth_layer)
        .layer(session_layer)
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

// async fn lib() -> impl IntoResponse {
//     // научные книги и статьи
//     // учебники и пособия
//     // публицистика
//     // проза и поэзия
//
//     let books = vec![
//         Book {
//             name: "book one".to_string(),
//             description: "description one".to_string(),
//             cover: "cover one".to_string(),
//         },
//         Book {
//             name: "book two".to_string(),
//             description: "description two".to_string(),
//             cover: "cover two".to_string(),
//         },
//     ];
//     HtmlTemplate(LibTemplate { books })
// }

// async fn blog() -> impl IntoResponse {
//     let posts = vec![
//         Post {
//             title: "post one".to_string(),
//             body: "body one".to_string(),
//         },
//         Post {
//             title: "post two".to_string(),
//             body: "body two".to_string(),
//         },
//     ];
//     HtmlTemplate(BlogTemplate { posts })
// }

//--------------------------------------------------------------------
// Books:
//--------------------------------------------------------------------
async fn books(State(state): State<Arc<HistoryState>>) -> Result<impl IntoResponse, HistoryError> {
    let books = Book::all(&state.db).await?;
    Ok(HtmlTemplate(BooksTemplate { books }))
}

// async fn books(State(state): State<Arc<HistoryState>>) -> Result<impl IntoResponse, HistoryError> {
//     let books = Book::all(&state.db).await?;
//     Ok(HtmlTemplate(BooksTemplate { books }))
// }
//--------------------------------------------------------------------

// async fn posts() -> impl IntoResponse {
//     let posts = vec![
//         Post {
//             title: "post one".to_string(),
//             body: "body one".to_string(),
//         },
//         Post {
//             title: "post two".to_string(),
//             body: "body two".to_string(),
//         },
//     ];
//     HtmlTemplate(PostsTemplate { posts })
// }

async fn login_form() -> impl IntoResponse {
    HtmlTemplate(LoginTemplate {})
}

async fn login(
    mut auth: Auth,
    State(state): State<Arc<HistoryState>>,
    Form(input): Form<LoginInput>,
) -> impl IntoResponse {
    if state.secret.eq(&input.secret) {
        let user = User::new(input.secret);
        auth.login(&user).await.unwrap();
        Redirect::to("/books")
    } else {
        Redirect::to("/login")
    }
}

async fn logout(mut auth: Auth) -> impl IntoResponse {
    auth.logout().await;
    Redirect::to("/")
}

async fn nothing() -> HistoryError {
    HistoryError::NotFound
}
