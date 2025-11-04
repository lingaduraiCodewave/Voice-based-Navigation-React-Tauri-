import { useNavigate } from "react-router-dom";
import VoiceButton from "../../component/VoiceButton";

const Home = () => {
  // const { isListening, transcript, error, toggleListening } =
  //   useVoiceNavigation();

  const navigate = useNavigate();

  const handleCommand = (text: string) => {
    const lower = text.toLowerCase();
    if (lower.includes("dashboard")) navigate("/dashboard");
    else if (lower.includes("desktop")) navigate("/desktop");
    else alert("Command not recognized 😅");
  };

  return (
    <div style={{ padding: "20px", fontFamily: "Arial, sans-serif" }}>
      <h1>Voice Navigation Demo</h1>

      <div style={{ padding: 20 }}>
        <h2>🎤 Voice Navigation Demo</h2>
        <VoiceButton onCommand={handleCommand} />
      </div>

      <div style={{ marginTop: "20px" }}>
        <h3>Voice Commands:</h3>
        <ul>
          <li>Say "dashboard" to go to Dashboard</li>
          <li>Say "home" to go to Home</li>
          <li>Say "settings" to go to Settings</li>
        </ul>
      </div>
    </div>
  );
};

export default Home;
