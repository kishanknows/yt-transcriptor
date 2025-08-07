use std::process::Command;
use serde_json::Value;

pub fn extract_youtube_ids(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("yt-dlp")
        .args(["--flat-playlist", "-J", url])
        .output()?;

    if !output.status.success(){
        return Err(format!("yt-dlp failed {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let json: Value = serde_json::from_slice(&output.stdout)?;
    let mut video_ids = Vec::new();

    // Playlist case
    if let Some(entries) = json.get("entries").and_then(|e| e.as_array()) {
        for entry in entries {
            if let Some(id) = entry.get("id").and_then(|v| v.as_str()){
                video_ids.push(id.to_string());
            }
        }
    }
    // Video case
    else if let Some(id) = json.get("id").and_then(|v| v.as_str()){
        video_ids.push(id.to_string());
    }

    Ok(video_ids)
}