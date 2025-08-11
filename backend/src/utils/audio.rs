use std::process::Command;
use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub fn download_audio(youtube_id: &str){
    let output = format!("downloads/{}.opus", youtube_id);
    let url = format!("https://www.youtube.com/watch?v={}", youtube_id);
    Command::new("yt-dlp")
        .args([
            "--no-playlist",
            "-x",
            "--audio-format", "opus",
            "--audio-quality", "9",
            "-o", &output,
            &url
        ])
        .status()
        .expect("failed to run yt-dlp");
}

pub fn convert_to_wav(youtube_id: &str) -> String {
    let input = format!("downloads/{}.opus", youtube_id);
    let output = format!("downloads/{}.wav", youtube_id);
    Command::new("ffmpeg")
        .args([
            "-i", &input,
            "-ar", "16000",
            "-ac", "1",
            "-f", "wav",
            "-acodec", "pcm_s16le",
            &output
        ])
        .status()
        .expect("failed to run ffmpeg");
    output
}

pub fn load_wav_as_f32(file_path: &str) -> Vec<f32> {
    let mut reader = hound::WavReader::open(Path::new(file_path)).unwrap();
    reader.samples::<i16>()
        .map(|s| s.unwrap() as f32/32768.0)
        .collect()
}

pub fn transcribe_wav(file_path: &str) -> String {
    let ctx = WhisperContext::new_with_params("llms/ggml-tiny.bin", WhisperContextParameters::default())
    .expect("failed to load model");

    let mut state = ctx.create_state().expect("failed to create state");

    let pcm_data = load_wav_as_f32(file_path);

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_translate(false);
    
    state.full(params, &pcm_data[..]).expect("failed to run whisper");

    let mut result = String::new();
    for i in 0..state.full_n_segments().unwrap() {
        result.push_str(&state.full_get_segment_text(i).unwrap());
    }
    result
}
