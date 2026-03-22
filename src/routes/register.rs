use std::sync::Arc;
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use axum::{Form, extract::State, http::StatusCode, response::Html};
use serde::Deserialize;
use tera::Context;
use crate::{db::Database, templates::TEMPLATES};

#[derive(Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn register_post(
    State(db): State<Arc<Database>>,
    Form(data): Form<RegisterData>,
) -> Result<Html<String>, StatusCode> {

    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let res = sqlx::query(
        "INSERT INTO User (email, username, password_hash) VALUES (?, ?, ?)"
    )
    .bind(&data.email)
    .bind(&data.username)
    .bind(&hash)
    .execute(&db.pool)
    .await;

    match res {
        Ok(_) => Ok(Html("User registered!".into())),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn register_get() -> Html<String> {
    TEMPLATES.render("register.html", &Context::new()).unwrap().into()
}
