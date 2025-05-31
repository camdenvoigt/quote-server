use axum::{Router, routing::get};
use sqlx::{SqlitePool, sqlite, migrate::MigrateDatabase};
use tokio::sync::RwLock;
use std::sync::Arc;

mod handlers;
mod quote;
mod templates;

struct AppState {
    db: SqlitePool
}

type ApplicationState = Arc<RwLock<AppState>>;

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let db_uri = "sqlite://db/quote-server";

    if !sqlite::Sqlite::database_exists(&db_uri).await? {
        std::fs::create_dir_all("quote-server")?;
        sqlite::Sqlite::create_database(&db_uri).await?
    }

    let db_conn = SqlitePool::connect(db_uri).await?;
    sqlx::migrate!().run(&db_conn).await?;

    let app_state = AppState { db: db_conn };
    let state = Arc::new(RwLock::new(app_state));

    let app = Router::new()
        .route("/", get(handlers::handle_get_index))
        .route("/quote", get(handlers::handle_get_quote))
        .route("/add-quote", get(handlers::handle_add_quote))
        .with_state(state);

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
