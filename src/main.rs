use rust_blog::app;

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        println!("{e}");
        ::std::process::exit(1);
    }

    let app = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
