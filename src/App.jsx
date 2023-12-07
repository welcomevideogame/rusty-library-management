import { useState, useEffect } from "react";
import { DataTable } from 'primereact/datatable';
import { Column } from 'primereact/column';

import logo from "./assets/library.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  
  const [activeFrame, setActiveFrame] = useState(null);

  const [mediaData, setMediaData] = useState([]);

  async function logIn(){
    await invoke('authenticate', {id: parseInt(username), password: password})
      .then((res) =>
        setIsLoggedIn(res)
      )
      .catch((e) => console.error(e))
  }

  const handleUsernameChange = (e) => {
    const value = e.currentTarget.value;
    if (/^[1-9]\d*$/.test(value)) {
      setUsername(value);
    } else if (value === '') {
      setUsername('');
    }
  };

  const handleFrameChange = (frame) => {
    setActiveFrame(frame);
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    console.log(username, password)
    logIn();
  };

  useEffect(() => {
    if (isLoggedIn && activeFrame === 'frame1') {
      invoke('get_media')
        .then((json) => {
          const data = JSON.parse(json);
          setMediaData(data);
        })
        .catch((error) => console.error('Error fetching media data:', error));
    }
  }, [isLoggedIn, activeFrame]);


  return (
    <div className="container">
      {!isLoggedIn ? (
        <>
          <h2>Library Management System</h2>
          
          <div className="row">
            <a href="https://google.com" target="_blank">
              <img src={logo} className="logo react" alt="logo logo" />
            </a>
          </div>

          <p>Please log in</p>

          <form className="row" onSubmit={handleSubmit}>
              <div>
                  <input
                      id="username-input"
                      onChange={handleUsernameChange}
                      value={username}
                      placeholder="Username"
                  />
              </div>
              <div>
                  <input
                      id="password-input"
                      onChange={(e) => setPassword(e.currentTarget.value)}
                      placeholder="Password"
                      type="password"
                  />
              </div>
              <button type="submit">Log In</button>
          </form>
        </>
      ) : (
        <>
        <div className="button-toolbar">
          <div className="button-group">
            <button
              className={`toolbar-button ${activeFrame === 'frame1' ? 'selected' : ''}`}
              onClick={() => handleFrameChange('frame1')}
            >
              Frame 1
            </button>
            <button
              className={`toolbar-button ${activeFrame === 'frame2' ? 'selected' : ''}`}
              onClick={() => handleFrameChange('frame2')}
            >
              Frame 2
            </button>
            <button
              className={`toolbar-button ${activeFrame === 'frame3' ? 'selected' : ''}`}
              onClick={() => handleFrameChange('frame3')}
            >
              Frame 3
            </button>
          </div>
        </div>
          
          {activeFrame === 'frame1' && 
          <div>
            <br></br>
            <DataTable value={mediaData} tableStyle={{ minWidth: '40rem' }}>
              <Column field="id" header="ID" />
              <Column field="media_type" header="Type" />
              <Column field="name" header="Title" />
              <Column field="borrowable" header="Available" />
              <Column field="vendor" header="Vendor" />
              <Column field="renter" header="Renter" />
            </DataTable>
          </div>}
          {activeFrame === 'frame2' && <div>Content of Frame 2</div>}
          {activeFrame === 'frame3' && <div>Content of Frame 3</div>}          
        </>
      )}
    </div>
  );
}

export default App;
