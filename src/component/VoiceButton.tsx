import { useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { writeFile, BaseDirectory } from "@tauri-apps/plugin-fs";
import { appLocalDataDir, join } from "@tauri-apps/api/path";
import RecordRTC from "recordrtc";

export default function VoiceButton({
  onCommand,
}: {
  onCommand: (text: string) => void;
}) {
  const [isRecording, setIsRecording] = useState(false);
  const recorderRef = useRef<RecordRTC | null>(null);

  async function startRecording() {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });

    const recorder: RecordRTC = new RecordRTC(stream, {
      type: "audio",
      mimeType: "audio/wav",
      recorderType: RecordRTC.StereoAudioRecorder,
      desiredSampRate: 16000,
      numberOfAudioChannels: 1,
    });

    recorder.startRecording();
    recorderRef.current = recorder;
    setIsRecording(true);
  }

  async function stopRecording() {
    return new Promise((resolve) => {
      const recorder = recorderRef.current;
      if (!recorder) {
        console.error(" Recorder not found!");
        resolve(null);
        return;
      }

      recorder.stopRecording(async () => {
        const blob = recorder.getBlob();
        const arrayBuffer = await blob.arrayBuffer();
        const buffer = new Uint8Array(arrayBuffer);

        const dir = await appLocalDataDir();
        const audioPath = await join(dir, "voice.wav");

        await writeFile("voice.wav", buffer, {
          baseDir: BaseDirectory.AppLocalData,
        });

        resolve(audioPath);
      });

      setIsRecording(false);
    });
  }

  async function handleVoice() {
    if (!isRecording) {
      startRecording();
    } else {
      const audioPath = await stopRecording();
      if (!audioPath) return;

      const text = (await invoke("recognize_audio", {
        path: audioPath,
      })) as string;
      console.log("Recognized:", text);
      onCommand(text);
    }
  }

  return (
    <button
      onClick={handleVoice}
      style={{
        background: isRecording ? "#e74c3c" : "#2ecc71",
        border: "none",
        padding: "10px 20px",
        color: "white",
        borderRadius: "8px",
        fontSize: "16px",
      }}
    >
      {isRecording ? "Stop 🎙️" : "Start Voice"}
    </button>
  );
}
