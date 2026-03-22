use std::sync::Arc;

use axum::{extract::{Path, State}, response::Html};
use crate::{Post, TEMPLATES};
use tera::Context;
use crate::db::Database;

pub async fn get_posts(State(db): State<Arc<Database>>) -> Html<String> {
    let mut context = Context::new();

    let posts = match sqlx::query_as::<_, Post>("SELECT * from Post ORDER BY post_id DESC")
        .fetch_all(&db.pool)
        .await {
            Ok(p) => p,
            Err(e) => {
                println!("{e}");
                return TEMPLATES.render("notfound.html", &context).unwrap().into();
            }
        };

    context.insert("posts", &posts);

    TEMPLATES.render("posts.html", &context).unwrap().into()
}

pub async fn get_post(Path(post_id): Path<i32>, State(db): State<Arc<Database>>) -> Html<String> {
    let mut context = Context::new();   

    let mut post = match sqlx::query_as::<_, Post>("SELECT * from Post WHERE post_id=?")
        .bind(post_id)
        .fetch_one(&db.pool)
        .await {
            Ok(p) => p,
            Err(e) => {
                println!("{e}");
                return TEMPLATES.render("notfound.html", &context).unwrap().into();
            }
        };

    post.contents_to_html();

    context.insert("post", &post);
    TEMPLATES.render("post.html", &context).unwrap().into()
}
