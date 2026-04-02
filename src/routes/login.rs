use std::sync::Arc;
use axum::{Form, extract::State, http::{StatusCode, header}, response::{Html, IntoResponse, Response}};
use tera::Context;
use crate::{TEMPLATES, auth::{self, SignInData}, db::Database};

pub async fn login_post(
    State(db): State<Arc<Database>>,
    Form(data): Form<SignInData>
) -> impl IntoResponse {
    let data = match auth::sign_in(data, db).await {
        Ok(d) => d,
        Err(e) => return e.into_response()
    };

    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, format!("auth_token={}; HttpOnly; Path=/", data))
        .body(axum::body::Body::empty())
        .unwrap()
}

pub async fn login_get() -> impl IntoResponse {
    Html(TEMPLATES.render("login.html", &Context::default()).unwrap())
}
