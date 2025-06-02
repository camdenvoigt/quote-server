use crate::*;
use crate::quote::Quote;
use axum::{response, response::IntoResponse, http, extract::{State, Path}};
use utoipa::OpenApi;
use utoipa_axum::{routes, router::OpenApiRouter};


pub fn get_router() -> OpenApiRouter<ApplicationState> {
    OpenApiRouter::new()
    .routes(routes!(handle_add_quote))
    .routes(routes!(handle_get_quote))
}

#[utoipa::path(
    get,
    path = "/add-quote",
    responses(
        (status = 200, description = "Successfully added to database", body = String),
        (status = 500, description = "Insert into database failed")
    )
)]
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

#[utoipa::path(
    get,
    path = "/quote/{id}",
    params(
        ("id", description = "Quote id"),
    ),
    responses(
        (status = 200, description = "Get Quote success", body = Quote),
        (status = 404, description = "Could not find quote")
    )
)]
pub async fn handle_get_quote(State(app_state) : State<ApplicationState>, Path(id) : Path<i64>) -> anyhow::Result<response::Response, http::StatusCode> {
    let app_reader = app_state.read().await;
    let db = &app_reader.db;

    let quote_res = quote::get(&db, id).await;

    match quote_res {
        Ok(quote) => Ok(quote.into_response()),
        Err(_) => Err(http::StatusCode::NOT_FOUND)
    }
}

#[derive(OpenApi)]
#[openapi(
    info(description = "Quote Api"),
)]
pub struct ApiDoc;
