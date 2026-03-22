use axum::response::Html;
use tera::Context;
use crate::TEMPLATES;

pub async fn index() -> Html<String> {
    TEMPLATES.render("index.html", &Context::default()).unwrap().into()
}
