use axum::{routing::{post}, Router};
use crate::handlers::{transcribe};

pub fn youtube_routes() -> Router {
    Router::new()
        .route("/youtube/transcribe", post(transcribe))
}