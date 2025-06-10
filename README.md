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

# Docker
Install Docker.

Build Image Locally
`docker build . -t quote-server`

Run container
`docker run -p 3000:3000 quote-server`

# Credit
This project was made with heavy influence from https://github.com/pdx-cs-rust-web/knock-knock-2. It also used and references many examples of code from rust crate documentation especially axum.
Use Cargo.toml as a reference.

# Reflection
Most things in the project went pretty smoothly. I have worked on web servers just like this many times so it was mostly a matter of learning the new ways to do things in Rust. I found the rust version of these libraries surprisingly easy to use. A bit more overhead then some other languages but honestly not much more. The most interesting things I learned were with sqlx. Preparing and migrating queries were things I had heard of before but never done myself so learning that was interesting. 

One area of trouble was docker (as always). I got the container running successfully but running it would never let me connect to the running server for some reason. I was using the port binding `-p 3000:3000`.

# License
This work is made available under the "Apache 2.0 or MIT License". See the file LICENSE.txt in this distribution for license terms.
