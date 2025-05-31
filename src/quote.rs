use serde::{Serialize, Deserialize};
use axum::response::{IntoResponse, Response};
use axum::http;
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub quote_id: i64,
    pub quote: String,
    pub author: String,
}

impl Quote {
    pub fn new(quote: String, author: String) -> Self {
        Self { quote_id: -1, quote, author }
    }

    pub async fn save_to_db(&self, db: &SqlitePool) -> anyhow::Result<i64> {
        let id = sqlx::query!(
        "
         INSERT INTO quotes (quote, author) VALUES(?1, ?2);
        ",
        self.quote,
        self.author
        )
        .execute(db)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}

impl IntoResponse for Quote {
    fn into_response(self) -> Response {
        (http::StatusCode::OK, axum::Json(&self)).into_response()
    }
}

pub async fn get(db: &SqlitePool, id: i64) -> anyhow::Result<Quote> {
    let quote = sqlx::query_as!(Quote,
        "
        SELECT * FROM quotes WHERE ?1 = quote_id;
        ",
        id
    )
    .fetch_one(db)
    .await?;

    Ok(quote)
}
