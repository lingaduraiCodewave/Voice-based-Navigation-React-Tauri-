import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { listen } from "@tauri-apps/api/event";

export const useVoiceNavigation = () => {
  const navigate = useNavigate();
  const [transcript, setTranscript] = useState("");
  const [isListening, setIsListening] = useState(true); // Vosk runs continuously

  useEffect(() => {
    let unlisten: any;

    listen<string>("voice-command", (event) => {
      const cmd = event.payload.toLowerCase();
      console.log("Voice Command Received:", cmd);
      setTranscript(cmd);

      // Handle voice navigation commands
      if (cmd.includes("dashboard")) navigate("/dashboard");
      else if (cmd.includes("home")) navigate("/");
      else if (cmd.includes("settings")) navigate("/settings");
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) unlisten();
    };
  }, [navigate]);

  return {
    isListening,
    transcript,
    error: null, // Vosk handles errors internally
    startListening: () => {}, // Vosk runs continuously
    stopListening: () => {}, // Vosk runs continuously
    toggleListening: () => {}, // Vosk runs continuously
  };
};
