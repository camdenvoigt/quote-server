use axum::{Router, routing::get};
use sqlx::{SqlitePool, sqlite, migrate::MigrateDatabase};
use tokio::sync::RwLock;
use std::sync::Arc;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::ApiDoc;
use utoipa::OpenApi;
use clap::Parser;

extern crate log;

use simple_logger;

mod handlers;
mod quote;
mod templates;
mod api;

/// A webserver to get and display famous quotes
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// A path to a csv to initialize the quote database from
    #[arg(short, long, name="init-from")]
    init_from: Option<std::path::PathBuf>
}

struct AppState {
    db: SqlitePool
}

type ApplicationState = Arc<RwLock<AppState>>;

async fn init_db(path : std::path::PathBuf, db_conn : &SqlitePool) -> anyhow::Result<()> {
   let mut rdr = csv::Reader::from_path(path)?;
    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        let quote = quote::Quote {
            quote_id: i as i64,
            quote: record.get(0).unwrap().to_string(),
            author: record.get(1).unwrap().to_string(),
        };
        quote.save_to_db(db_conn).await?;
    }
    Ok(())
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // setup logger
    simple_logger::SimpleLogger::new().env().init().unwrap();

    // setup database
    let db_uri = "sqlite://db/quote-server";

    if !sqlite::Sqlite::database_exists(&db_uri).await? {
        std::fs::create_dir_all("quote-server")?;
        sqlite::Sqlite::create_database(&db_uri).await?
    }

    let db_conn = SqlitePool::connect(db_uri).await?;
    sqlx::migrate!().run(&db_conn).await?;

    if let Some(init_from) = args.init_from {
        init_db(init_from, &db_conn).await?;
    }

    // get api
    let (api_router, openapi) = api::get_router().split_for_parts();

    // setup swagger_ui
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", openapi);

    // Set up the application state
    let app_state = AppState { db: db_conn };
    let state = Arc::new(RwLock::new(app_state));

    // Setup the Server
    let app = Router::new()
        .route("/", get(handlers::handle_get_index))
        .merge(api_router)
        .merge(swagger_ui)
        .with_state(state);

    // Start the server
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
