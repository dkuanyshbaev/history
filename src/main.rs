use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{env, net::SocketAddr, sync::Arc};

//-----------------------------------------------------
use std::collections::HashMap;

use axum_login::{
    axum_sessions::async_session::MemoryStore as SessionMemoryStore, extractors::AuthContext,
    memory_store::MemoryStore as AuthMemoryStore,
};
use tokio::sync::RwLock;
//-----------------------------------------------------
use axum::Extension;
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    secrecy::SecretVec,
    AuthLayer, AuthUser, RequireAuthorizationLayer, SqliteStore,
};
use rand::Rng;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Debug, Clone, sqlx::FromRow)]
struct User {
    id: usize,
    name: String,
    password_hash: String,
    role: Role,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Role {
    User,
    Admin,
}

impl AuthUser<usize, Role> for User {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }

    fn get_role(&self) -> Option<Role> {
        Some(self.role.clone())
    }
}

// type AuthContext = axum_login::extractors::AuthContext<i64, User, SqliteStore<User>>;
// type Auth = AuthContext<usize, User, AuthMemoryStore<usize, User>>;
// type RequireAuth = RequireAuthorizationLayer<usize, User>;

type Auth = AuthContext<usize, User, AuthMemoryStore<usize, User>, Role>;
type RequireAuth = RequireAuthorizationLayer<usize, User, Role>;

//-----------------------------------------------------------

struct HistoryState {
    secret: String,
}

struct Book {
    name: String,
    description: String,
    cover: String,
}

struct Post {
    title: String,
    body: String,
}

#[tokio::main]
async fn main() {
    let secret = env::var("SECRET").expect("SECRET must be set!");
    let state = Arc::new(HistoryState { secret });

    //-------------------------------------------------
    let secret = rand::thread_rng().gen::<[u8; 64]>();
    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret);

    //------------------
    let store = Arc::new(RwLock::new(HashMap::default()));
    let user = User {
        id: 42,
        name: "Denis".to_string(),
        password_hash: "password".to_string(),
        role: Role::Admin,
    };
    store.write().await.insert(user.get_id(), user);

    let user_store = AuthMemoryStore::new(&store);
    let auth_layer = AuthLayer::new(user_store, &secret);
    //------------------

    // let pool = SqlitePoolOptions::new()
    //     .connect("db/history.db")
    //     .await
    //     .unwrap();
    //
    // let user_store = SqliteStore::<User>::new(pool);
    // let auth_layer = AuthLayer::new(user_store, &secret);
    //-------------------------------------------------

    let history = Router::new()
        // .route("/", get(home))
        // .route("/lib", get(lib))
        // .route("/blog", get(blog))
        .route("/about", get(about))
        //--------------------
        .route("/protected", get(protected_handler))
        // .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        .route_layer(RequireAuth::login())
        .route("/login", get(login_handler))
        .route("/logout", get(logout_handler))
        .route("/", get(home))
        .layer(auth_layer)
        .layer(session_layer)
        //--------------------
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

async fn about() -> impl IntoResponse {
    HtmlTemplate(AboutTemplate {})
}

//-----------------------------------------------------------

async fn protected_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Logged in as: {}", user.name)
}

async fn login_handler(mut auth: Auth) {
    // let pool = SqlitePoolOptions::new()
    //     .connect("db/history.db")
    //     .await
    //     .unwrap();
    // let mut conn = pool.acquire().await.unwrap();
    // let user: User = sqlx::query_as("select * from users where id = 1")
    //     .fetch_one(&mut conn)
    //     .await
    //     .unwrap();
    let user = User {
        id: 42,
        name: "Denis".to_string(),
        password_hash: "password".to_string(),
        role: Role::Admin,
    };
    auth.login(&user).await.unwrap();
}

async fn logout_handler(mut auth: Auth) {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;
}

//-----------------------------------------------------------

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
