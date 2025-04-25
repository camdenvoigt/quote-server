use axum::{Router, routing::get};

mod handlers;
mod quote;
mod templates;

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(handlers::handle_get_index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = serve().await {
        eprintln!("quote-server: error: {}", err);
        std::process::exit(1);
    }
}
