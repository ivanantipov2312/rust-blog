use axum::response::{Html, IntoResponse};
use tera::Context;
use crate::TEMPLATES;

pub async fn index() -> impl IntoResponse {
    Html(TEMPLATES.render("index.html", &Context::default()).unwrap())
}
