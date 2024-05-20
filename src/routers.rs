use axum::Json;
use serde::Deserialize;

use crate::generator::GENERATOR;

/// Body of the `POST /`
#[derive(Deserialize)]
pub struct GenerateBody {
    /// Start of the text that the Markov Chain will complete
    start: String,
    /// Number of tokens that the response will contain
    len: u8,
}

/// `POST /`
pub async fn generate(Json(body): Json<GenerateBody>) -> String {
    GENERATOR.generate_start(&body.start, body.len as usize)
}
