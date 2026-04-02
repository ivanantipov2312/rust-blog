use std::sync::Arc;

use axum::{Extension, Form, extract::State, response::{Html, IntoResponse, Redirect}};
use serde::Deserialize;
use tera::Context;

use crate::{TEMPLATES, auth::CurrentUser, db::Database, error::AppError};

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
) -> impl IntoResponse {

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
        Ok(_) => Redirect::to("/posts").into_response(),
        Err(_) => AppError::Internal.into_response()
    }
}

pub async fn add_post_get() -> impl IntoResponse {
    Html(TEMPLATES.render("add_post.html", &Context::default()).unwrap())
}
