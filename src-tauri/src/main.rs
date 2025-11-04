#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use vosk::{Model, Recognizer};
use std::fs::File;
use std::io::Read;
use serde_json::Value;

#[command]
fn recognize_audio(path: String) -> String {
     println!("Got audio path: ");
    let model_path = "vosk-model-small-en-us-0.15";
    let model = Model::new(model_path).expect("Failed to load Vosk model");
    let mut recognizer = Recognizer::new(&model, 16000.0).expect("Failed to create recognizer");

    // Read the WAV audio
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("❌ Could not open {}: {}", path, e);
            return "error".to_string();
    }
    };
    println!("Got audio path: ");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    // Convert bytes → i16 samples
    let samples: Vec<i16> = buffer
        .chunks_exact(2)
        .map(|b| i16::from_le_bytes([b[0], b[1]]))
        .collect();

    recognizer.accept_waveform(&samples).unwrap();

    // ✅ Get the final recognition result as JSON string
    let result = recognizer.final_result();
    let json_str = serde_json::to_string(&result).unwrap_or_default();

    // Parse text from JSON
    let json_value: Value = serde_json::from_str(&json_str).unwrap_or_default();
    json_value["text"].as_str().unwrap_or("").to_string()
}

fn main() {
    println!("Got audio path: ");
    tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![recognize_audio])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
