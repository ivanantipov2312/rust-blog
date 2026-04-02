use crate::{
    auth::{jwt::decode_jwt, retrieve_user_by_email},
    db::Database,
};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, header},
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use std::sync::Arc;

pub async fn authorization_middleware(
    State(db): State<Arc<Database>>,
    mut req: Request,
    next: Next,
) -> Response<Body> {
    let cookies = match req
        .headers()
        .get(header::COOKIE)
        .and_then(|h| h.to_str().ok())
    {
        Some(c) => c,
        None => return Redirect::to("/login").into_response(),
    };

    let token = cookies.split(';').find_map(|c| {
        let mut parts = c.trim().splitn(2, '=');
        if parts.next()? == "auth_token" {
            parts.next()
        } else {
            None
        }
    });

    let token = match token {
        Some(t) => t,
        None => return Redirect::to("/login").into_response(),
    };

    let token_data = match decode_jwt(token.to_string()) {
        Ok(data) => data,
        Err(_) => return Redirect::to("/login").into_response(),
    };

    let current_user = match retrieve_user_by_email(&token_data.claims.email, &db).await {
        Some(user) => user,
        None => return Redirect::to("/login").into_response(),
    };
    req.extensions_mut().insert(current_user);
    next.run(req).await
}
