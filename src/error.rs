use axum::{response::Html};
use tera::Context;
use crate::TEMPLATES;

pub enum AppError {
    AuthFailed,
    UserNotFound,
    PostNotFound,
    Internal,
}

impl Into<Html<String>> for AppError {
    fn into(self) -> Html<String> {
         match self {
            Self::AuthFailed => TEMPLATES.render("auth_error.html", &Context::new()).unwrap().into(),
            Self::UserNotFound => TEMPLATES.render("user_not_found.html", &Context::new()).unwrap().into(),
            Self::PostNotFound => TEMPLATES.render("post_not_found.html", &Context::new()).unwrap().into(),
            Self::Internal => TEMPLATES.render("internal_error.html", &Context::new()).unwrap().into()
        }       
    }
}
