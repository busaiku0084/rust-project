use axum::{
    routing::{get, post},
    Router,
    serve,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handlers;
mod models;
mod storage;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::show_form).post(handlers::submit_post))
        .route("/edit/:id", get(handlers::edit_form).post(handlers::edit_submit))
        .route("/delete/:id", post(handlers::delete_post));

    // Axum 0.7+ では TcpListener + serve(app) が推奨
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Listening on http://localhost:3000");

    serve(listener, app).await.unwrap();
}
