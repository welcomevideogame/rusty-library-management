import { useState, useEffect } from "react";
import { DataTable } from 'primereact/datatable';
import { Column } from 'primereact/column';

import logo from "./assets/library.svg";
import { invoke } from "@tauri-apps/api/tauri";
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
