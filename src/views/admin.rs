use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use std::sync::Arc;

use crate::{Auth, HistoryState, HtmlTemplate, LoginInput, User};

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct LoginTemplate;

pub async fn form() -> impl IntoResponse {
    HtmlTemplate(LoginTemplate {})
}

pub async fn login(
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

pub async fn logout(mut auth: Auth) -> impl IntoResponse {
    auth.logout().await;
    Redirect::to("/")
}
