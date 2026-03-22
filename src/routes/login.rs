use std::sync::Arc;
use axum::{http::header, Form, extract::State, http::StatusCode, response::{Html, Response}};
use tera::Context;
use crate::{TEMPLATES, auth::{self, SignInData}, db::Database};

pub async fn login_post(
    State(db): State<Arc<Database>>,
    Form(data): Form<SignInData>
) -> Result<Response, Html<String>> {
    let data = match auth::sign_in(data, db).await {
        Ok(d) => d,
        Err(e) => return Err(e.into())
    };

    let response = Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, format!("auth_token={}; HttpOnly; Path=/", data))
        .body(axum::body::Body::empty())
        .unwrap();

    Ok(response)
}

pub async fn login_get() -> Html<String> {
    TEMPLATES.render("login.html", &Context::default()).unwrap().into()
}
