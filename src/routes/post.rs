use crate::{TEMPLATES, db::Database, error::AppError};
use axum::{
    extract::{Path, State},
    response::Html,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use tera::Context;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Post {
    pub post_id: i32,
    pub username: String,
    pub title: String,
    pub contents: String,
}

pub async fn get_posts(State(db): State<Arc<Database>>) -> Html<String> {
    let mut context = Context::new();

    let posts = match sqlx::query_as::<_, Post>(
        "
        SELECT Post.post_id,Post.title,Post.contents,User.username from Post
        JOIN User ON Post.user_id = User.user_id
        ORDER BY post_id DESC",
    )
    .fetch_all(&db.pool)
    .await
    {
        Ok(p) => p,
        Err(e) => {
            println!("{e}");
            return AppError::PostNotFound.into()
        }
    };

    context.insert("posts", &posts);

    TEMPLATES.render("posts.html", &context).unwrap().into()
}

pub async fn get_post(
    Path(post_id): Path<i32>,
    State(db): State<Arc<Database>>
) -> Html<String> {
    let mut context = Context::new();

    let mut post = match sqlx::query_as::<_, Post>(
        "
        SELECT Post.post_id,Post.title,Post.contents,User.username from Post
        JOIN User ON Post.user_id = User.user_id
        WHERE post_id=?
        ",
    )
    .bind(post_id)
    .fetch_one(&db.pool)
    .await
    {
        Ok(p) => p,
        Err(e) => {
            println!("{e}");
            return AppError::PostNotFound.into();
        }
    };

    post.contents = crate::utils::markdown_to_html(&post.contents);

    context.insert("post", &post);
    TEMPLATES.render("post.html", &context).unwrap().into()
}
