# Rust Quote Server
Author: Camden Jackson Voigt.

A small web server written using rust axum to learn the basics of rust for the web. It uses Axum as the main server framework, an sqlite db, and will use Leptos for the front end.

# Running Quote Server
To run in release mode run
`cargo run --release`

## Run Quote server and initialize with quotes
`cargo run --release -- -i see-quotes/quotes.csv`

# Sqlx
Migrations are run when the server starts

Prepare new queries
`cargo sqlx prepare`

# Credit
This project was made with heavy influence from https://github.com/pdx-cs-rust-web/knock-knock-2. It also used and references many examples of code from rust crate documentation especially axum.
Use Cargo.toml as a reference.

# License
This work is made available under the "Apache 2.0 or MIT License". See the file LICENSE.txt in this distribution for license terms.
