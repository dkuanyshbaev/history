use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    extractors::AuthContext,
    memory_store::MemoryStore as AuthMemoryStore,
    secrecy::SecretVec,
    AuthLayer, AuthUser, RequireAuthorizationLayer,
};
use rand::Rng;
use std::{collections::HashMap, env, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct User {
    id: usize,
    name: String,
    password_hash: String,
    role: Role,
}

impl User {
    pub fn new_admin(secret: String) -> Self {
        Self {
            id: 42,
            name: "Admin".to_string(),
            password_hash: secret,
            role: Role::Admin,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Role {
    _User,
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

type Auth = AuthContext<usize, User, AuthMemoryStore<usize, User>, Role>;
type RequireAuth = RequireAuthorizationLayer<usize, User, Role>;

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
    let session_secret = rand::thread_rng().gen::<[u8; 64]>();
    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &session_secret);

    let store = Arc::new(RwLock::new(HashMap::default()));
    let user = User::new_admin(secret);
    store.write().await.insert(user.get_id(), user);
    let user_store = AuthMemoryStore::new(&store);
    let auth_layer = AuthLayer::new(user_store, &session_secret);

    let history = Router::new()
        .route("/protected", get(protected_handler))
        .route_layer(RequireAuth::login_with_role(Role::Admin..))
        .route("/login", get(login))
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

async fn protected_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Logged in as: {}", user.name)
}

async fn login(mut auth: Auth) {
    let secret = "test".to_string();
    let user = User::new_admin(secret);
    auth.login(&user).await.unwrap();
}

async fn logout(mut auth: Auth) {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;
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
