mod markov;
mod routers;
use std::{
    env,
    fs::{self, read_to_string},
    net::SocketAddr,
};

use axum::{routing::post, Json, Router};
use markov::MarkovChain;

use once_cell::sync::Lazy;
use serde::Deserialize;

/// Markov Chain that will do the generation. Encapsulated in `Lazy` to allow it to be global.
static GENERATOR: Lazy<MarkovChain> = Lazy::new(|| train_markov());

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
        Lazy::force(&GENERATOR);
        println!("Generator is trained");
    }

    let app = Router::new().route("/", post(routers::generate));

    // Listens requests from port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening at port: {}", addr.port());
    axum::serve(listener, app).await.unwrap();
}

/// Makes necessary preperations and trains the Markov Chain
fn train_markov() -> MarkovChain {
    let home_dir = env::var("HOME").expect("HOME Environment Variable not found");
    let training_path = home_dir + "/markov_chain" + "/training";

    // Gets the paths of evey file and directory in the training_path.
    let tpaths =
        fs::read_dir(&training_path).expect(&format!("Can't read files from: {}", training_path));

    // Only the files remain
    let files = tpaths
        .filter_map(|f| f.ok())
        .filter(|f| match f.file_type() {
            Err(_) => false,
            Ok(f) => f.is_file(),
        });

    // Reads every file into a string
    let contents = files.filter_map(|f| read_to_string(f.path()).ok());

    // Creating the Markov Chain
    let markov_chain = contents.fold(MarkovChain::with_capacity(2, 8_000_000), |mut a, s| {
        a.add_text(&s);
        a
    });

    markov_chain
}
