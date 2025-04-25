use axum::{Router, routing::get};

mod handlers;
mod quote;
mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handlers::handle_get_index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
