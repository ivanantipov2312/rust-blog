mod error;
mod routes;
mod db;
mod auth;
mod utils;
mod templates;

use std::sync::Arc;
use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use db::Database;
pub use templates::TEMPLATES;

use crate::routes::fallback;

pub async fn app() -> Router {
    let state = Arc::new(Database::new("data/post_db.db").await.unwrap());

    let protected = Router::new()
        .route("/posts/new", get(routes::add_post_get).post(routes::add_post_post))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), auth::middleware::authorization_middleware));

    Router::new()
        .route("/", get(routes::index))
        .route("/posts", get(routes::get_posts))
        .route("/posts/{id}", get(routes::get_post))
        .route("/login", get(routes::login_get).post(routes::login_post))
        .route("/register", get(routes::register_get).post(routes::register_post))
        .merge(protected)
        .with_state(state)
        .nest_service("/static", ServeDir::new("static"))
        .fallback(fallback)
}
