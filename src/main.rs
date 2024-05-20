mod generator;
mod markov;
mod routers;

use axum::{routing::post, Router};
use once_cell::sync::Lazy;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load variables from `.env`.
    // It returns `Err` when `.env` is not found, but I don't always want a `.env` file to be present.
    let _ = dotenvy::dotenv();

    let port: u16 = std::env::var("PORT")
        .expect("PORT env var")
        .parse()
        .expect("PORT env var couldn't parse to 16 bit unsigned int");

    // Training Markov Chain before starting the server. The server won't start until training is finished.
    // If this step were to be omited, the training won't begin until a POST request to `/` is sent, due to how `Lazy` is evaluated.
    {
        println!("Training the Generator");
        // Forces `Lazy` to be evaluated.
        Lazy::force(&generator::GENERATOR);
        println!("Generator is trained");
    }

    let app = Router::new().route("/", post(routers::generate));

    // Listens requests from port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening at port: {}", addr.port());
    axum::serve(listener, app).await.unwrap();
}
