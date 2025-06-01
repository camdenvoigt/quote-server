use crate::*;
use axum::{response, response::IntoResponse, http, extract::State};

pub async fn handle_get_index() -> response::Html<String> {
    log::error!("Testing here");
    let quote = quote::Quote::new(
        String::from(
            "I can calculate the motion of heavenly bodies but not the madness of people.",
        ),
        String::from("Isaac Newton"),
    );
    let template = templates::IndexTemplate::quote(&quote);

    response::Html(template.to_string())
}

pub async fn handle_add_quote(State(app_state) : State<ApplicationState>) -> anyhow::Result<response::Response, http::StatusCode> {
    let app_reader = app_state.read().await;
    let db = &app_reader.db;

    let quote = quote::Quote::new(
        String::from(
            "I can calculate the motion of heavenly bodies but not the madness of people.",
        ),
        String::from("Isaac Newton"),
    );

    let quote_res = quote.save_to_db(&db).await;
    match quote_res {
        Ok(_) => Ok("Added to Database".into_response()),
        Err(err) => {
            log::error!("{}", err);
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_get_quote(State(app_state) : State<ApplicationState>) -> anyhow::Result<response::Response, http::StatusCode> {
    let id = 1;    
    let app_reader = app_state.read().await;
    let db = &app_reader.db;

    let quote_res = quote::get(&db, id).await;

    match quote_res {
        Ok(quote) => Ok(quote.into_response()),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
