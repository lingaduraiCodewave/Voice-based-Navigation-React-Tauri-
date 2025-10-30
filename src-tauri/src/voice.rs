use tauri::{AppHandle, Manager, Emitter};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use vosk::{Model, Recognizer};
use std::time::Duration;

/// Starts the continuous voice recognition process in a separate thread.
/// It uses cpal for audio input and vosk for transcription.
/// Emits 'voice-command' events to the frontend with partial results.
pub fn start_voice_recognition(app: AppHandle) {
    // 1. Setup Audio Host and Device
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device found");
    let config = device.default_input_config().unwrap();

    // 2. Setup Vosk Model and Recognizer
    // NOTE: Ensure the 'vosk-model-small' directory is correctly placed at 
    // src-tauri/models/vosk-model-small
    let model_path = app.path().resource_dir()
    .unwrap()
    .join("models")
    .join("vosk-model-small");
    let model = Model::new(model_path.to_string_lossy().to_string()).unwrap();

    let mut rec = Recognizer::new(&model, 16000.0).unwrap();

    // 3. Build and Play Audio Stream
    let stream = device.build_input_stream(
        &config.into(),
        // Data callback
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // Convert f32 samples to i16 for Vosk
            let audio_i16: Vec<i16> = data.iter().map(|&x| (x * i16::MAX as f32) as i16).collect();
            rec.accept_waveform(&audio_i16).unwrap();

            // Emit partial results to the Tauri frontend
            let result = rec.partial_result();
            if !result.partial.is_empty() {
                if let Err(e) = app.emit("voice-command", result.partial) {
                    eprintln!("Failed to emit voice-command event: {:?}", e);
                }
            }
        },
        // Error callback
        move |err| eprintln!("Error capturing audio: {:?}", err),
        None,
    ).unwrap();

    stream.play().unwrap();

    // Keep the thread alive so the stream continues to run
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}