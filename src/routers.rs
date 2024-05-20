use axum::Json;
use serde::Deserialize;

/// Body of the `POST /`
#[derive(Deserialize)]
struct GenerateBody {
    /// Start of the text that the Markov Chain will complete
    start: String,
    /// Number of tokens that the response will contain
    len: u8,
}

/// `POST /`
pub async fn generate(Json(body): Json<GenerateBody>) -> String {
    GENERATOR.generate_start(&body.start, body.len as usize)
}
