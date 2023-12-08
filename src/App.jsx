import { useState } from "react";
import "./App.css";
import SignIn from "./SignIn";

function App() {
  const [isLoggedIn, handleLoginSuccess] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  
  const [activeFrame, setActiveFrame] = useState(null);
  const [mediaData, setMediaData] = useState([]);
  const [rank, setRank] = useState("");

  const [buttonVisibility, setButtonVisibility] = useState({
    mediaButton: false,
    employeesButton: false,
    settingsButton: false
  });

  return (
    <div className="container">
      {!isLoggedIn ? (
        <>
        <SignIn onLoginSuccess={handleLoginSuccess} />
        </>
      ) : (
        <>
          Dashboard goes here
        </>
      )}
    </div>
  );
}

export default App;
