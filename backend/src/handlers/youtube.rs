use std::fs;
use axum::{response::IntoResponse, Json};
use crate::models::{TranscribeRequest, TranscribeResponse};
use crate::utils::{download_audio, convert_to_wav, extract_youtube_ids, transcribe_wav};

pub async fn transcribe(Json(payload): Json<TranscribeRequest>) -> impl IntoResponse {
    let TranscribeRequest { url } = &payload;
    let mut transcriptions = Vec::new();

    match extract_youtube_ids(url){
        Ok(ids) => {
            for (i, id) in ids.iter().enumerate() {
                
                println!("Processing: {}, {} of {}", id, i+1, ids.len());
                match download_audio(id){
                    Ok(status) => {
                        if status.success() {
                            let file_path = convert_to_wav(id);
                            let text = transcribe_wav(&file_path);
                            transcriptions.push(text);
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to process {}", e);
                    }
                }
                
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let _ = fs::remove_dir_all("downloads").expect("unable to delete downloads");
    println!("Transcription successful!");

    return Json(TranscribeResponse {
        status: "success".to_string(),
        message: format!("Transcribing audio from: {}", payload.url),
        transcription: transcriptions,
    });
}

