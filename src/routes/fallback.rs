use axum::{http::StatusCode, response::{Html, IntoResponse}};
use tera::Context;

use crate::TEMPLATES;

pub async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(TEMPLATES.render("not_found.html", &Context::new()).unwrap()))
}
