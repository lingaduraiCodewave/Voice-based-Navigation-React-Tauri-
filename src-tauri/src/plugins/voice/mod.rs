use tauri::{plugin::{Builder, TauriPlugin}, Runtime};
use vosk::{Model, Recognizer};
use std::fs::File;
use std::io::Read;
use serde_json::Value;

#[tauri::command]
fn recognize_audio(path: String) -> String {
    println!("✅ Loaded Vosk model successfully");
    let model_path = "vosk-model-small-en-us-0.15";
    let model = Model::new(model_path).expect("Failed to load Vosk model");
    let mut recognizer = Recognizer::new(&model, 16000.0).expect("Failed to create recognizer");

    let mut file = File::open(&path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let samples: Vec<i16> = buffer
        .chunks_exact(2)
        .map(|b| i16::from_le_bytes([b[0], b[1]]))
        .collect();

    recognizer.accept_waveform(&samples).unwrap();
    let result = recognizer.final_result();
    let json_str = serde_json::to_string(&result).unwrap_or_default();
    let json_value: Value = serde_json::from_str(&json_str).unwrap_or_default();

    json_value["text"].as_str().unwrap_or("").to_string()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("voice")
        .invoke_handler(tauri::generate_handler![recognize_audio])
        .build()
}
