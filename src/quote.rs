use serde::Deserialize;

#[derive(Deserialize)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

impl Quote {
    pub fn new(text: String, author: String) -> Self {
        Self { text, author }
    }
}
