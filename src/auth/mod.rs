mod jwt;
pub mod middleware;

use std::sync::Arc;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::prelude::FromRow;

use crate::{auth::jwt::encode_jwt, db::Database};

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[derive(Clone, FromRow)]
pub struct CurrentUser {
    pub user_id: i32,
    pub email: String,
    pub password_hash: String
}

pub async fn sign_in(
    user_data: SignInData,
    db: Arc<Database>
) -> Result<String, StatusCode> {
    let user = match retrieve_user_by_email(&user_data.email, &db).await {
        Some(u) => u,
        None => {
            println!("No user with this email exists!");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(h) => h,
        Err(_) => {
            println!("Password hash doesn't have a valid format!");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let argon2 = Argon2::default();
    if argon2.verify_password(user_data.password.as_bytes(), &parsed_hash).is_err() {
        println!("Hashed do not match!");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let encoded = encode_jwt(&user.email)?;
    Ok(encoded)
}

async fn retrieve_user_by_email(email: &str, db: &Database) -> Option<CurrentUser> {
    let u = match sqlx::query_as::<_, CurrentUser>("SELECT User.user_id,User.email,User.password_hash from User where email=?")
        .bind(email)
        .fetch_one(&db.pool)
        .await {
            Ok(u) => u,
            Err(e) => {
                println!("{e}");
                return None;
            },
        };

    Some(u)
}
