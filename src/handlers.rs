use crate::*;
use axum::response;

pub async fn handle_get_index() -> response::Html<String> {
    let quote = quote::Quote::new(
        String::from(
            "I can calculate the motion of heavenly bodies but not the madness of people.",
        ),
        String::from("Isaac Newton"),
    );
    let template = templates::IndexTemplate::quote(&quote);

    response::Html(template.to_string())
}

