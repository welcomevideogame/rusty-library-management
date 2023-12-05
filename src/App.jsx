import { useState, useEffect } from "react";
import logo from "./assets/library.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

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

  const handleSubmit = (e) => {
    e.preventDefault();
    console.log(username, password)
    logIn();
  };


  return (
    <div className="container">
      <h2>Library Management System</h2>

      {!isLoggedIn ? (
        <>
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
        <div>
          <h3>Logged in: {username}!</h3>
          <button onClick={functionOne}>Button 1</button>
        </div>
      )}
    </div>
  );
}

export default App;
