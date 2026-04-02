use axum::{http::StatusCode, response::{Html, IntoResponse, Response}};
use tera::Context;
use crate::TEMPLATES;

pub enum AppError {
    AuthFailed,
    UserNotFound,
    PostNotFound,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, body) = match self {
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, Html(TEMPLATES.render("internal_error.html", &Context::new()).unwrap())),
            AppError::UserNotFound => (StatusCode::UNAUTHORIZED, Html(TEMPLATES.render("user_not_found.html", &Context::new()).unwrap())),
            AppError::PostNotFound => (StatusCode::NOT_FOUND, Html(TEMPLATES.render("post_not_found.html", &Context::new()).unwrap())),
            AppError::AuthFailed => (StatusCode::UNAUTHORIZED, Html(TEMPLATES.render("auth_error.html", &Context::new()).unwrap()))
        };

        (code, body).into_response()
    }
}
