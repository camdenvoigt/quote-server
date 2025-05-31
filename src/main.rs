use axum::{Router, routing::get};
use sqlx::{SqlitePool, sqlite, migrate::MigrateDatabase};

mod handlers;
mod quote;
mod templates;

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let db_uri = "sqlite://db/quote-server";

    if !sqlite::Sqlite::database_exists(&db_uri).await? {
        std::fs::create_dir_all("quote-server")?;
        sqlite::Sqlite::create_database(&db_uri).await?
    }

    let db_conn = SqlitePool::connect(db_uri).await?;
    sqlx::migrate!().run(&db_conn).await?;

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
