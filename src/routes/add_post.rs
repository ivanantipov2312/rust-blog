use std::sync::Arc;

use axum::{Extension, Form, extract::State, http::StatusCode, response::Html};
use serde::Deserialize;
use tera::Context;

use crate::{TEMPLATES, auth::CurrentUser, db::Database};

#[derive(Deserialize)]
pub struct NewPostData {
    pub title: String,
    pub slug: String,
    pub contents: String,
}

pub async fn add_post_post(
    State(db): State<Arc<Database>>,
    Extension(user): Extension<CurrentUser>,
    Form(data): Form<NewPostData>,
) -> Result<Html<String>, StatusCode> {

    let res = sqlx::query(
        "INSERT INTO Post (user_id, title, slug, contents) VALUES (?, ?, ?, ?)"
    )
    .bind(user.user_id)
    .bind(&data.title)
    .bind(&data.slug)
    .bind(&data.contents)
    .execute(&db.pool)
    .await;

    match res {
        Ok(_) => Ok(Html("Post created!".into())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_post_get() -> Html<String> {
    TEMPLATES.render("add_post.html", &Context::default()).unwrap().into()
}
