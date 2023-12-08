import { useState, useEffect } from "react";
import { DataTable } from 'primereact/datatable';
import { Column } from 'primereact/column';

import logo from "./assets/library.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import SignIn from "./SignIn";
import Dashboard from "./Dashboard";

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

  useEffect(() => {
    if (isLoggedIn && activeFrame === 'media') {
      invoke('get_media')
        .then((json) => {
          const data = JSON.parse(json);
          setMediaData(data);
        })
        .catch((error) => console.error('Error fetching media data:', error));
    }
  }, [isLoggedIn, activeFrame]);

  useEffect(() => {
    if (isLoggedIn) {
      invoke('get_rank')
        .then((rank) => {
          setRank(rank);
          handleRankDisplay(rank); // Update to pass rank
        })
        .catch((error) => console.error('Error fetching rank data:', error));
    }
  }, [isLoggedIn]);

  const handleRankDisplay = (rank) => {
    switch(rank){
      case 'Basic':
      case 'User':
        setButtonVisibility({ mediaButton: true, employeesButton: false, settingsButton: false });
        break;
      case 'Manager':
        setButtonVisibility({ mediaButton: true, employeesButton: true, settingsButton: false });
        break;
      case 'Admin':
      case 'Dev':
        setButtonVisibility({ mediaButton: true, employeesButton: true, settingsButton: true });
        break;
      case 'None':
      default:
        setButtonVisibility({ mediaButton: false, employeesButton: false, settingsButton: false });
        break;
    }
  }

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
