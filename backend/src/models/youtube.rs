use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct TranscribeRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct TranscribeResponse {
    pub status: String,
    pub message: String,
    pub transcription: Vec<String>,
}