use std::sync::Arc;
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use axum::{Form, body::Body, extract::State, http::Response, response::{Html, IntoResponse, Redirect}};
use serde::Deserialize;
use tera::Context;
use crate::{db::Database, error::AppError, templates::TEMPLATES};

#[derive(Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn register_post(
    State(db): State<Arc<Database>>,
    Form(data): Form<RegisterData>,
) -> Result<Response<Body>, Html<String>> {

    let salt = SaltString::generate(&mut OsRng);

    let hash = match Argon2::default()
        .hash_password(data.password.as_bytes(), &salt) {
            Ok(h) => h,
            Err(_) => return Err(AppError::Internal.into())
        }.to_string();

    let res = sqlx::query(
        "INSERT INTO User (email, username, password_hash) VALUES (?, ?, ?)"
    )
    .bind(&data.email)
    .bind(&data.username)
    .bind(&hash)
    .execute(&db.pool)
    .await;

    match res {
        Ok(_) => Ok(Redirect::to("/login").into_response()),
        Err(_) => Err(AppError::Internal.into()),
    }
}

pub async fn register_get() -> Html<String> {
    TEMPLATES.render("register.html", &Context::new()).unwrap().into()
}
