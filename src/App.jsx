import { useState, useEffect } from "react";
import logo from "./assets/library.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [databaseManager, setDatabaseManager] = useState(null);

  useEffect(() => {
    const initializeDatabaseManager = async () => {
      const manager = await invoke("create_db_manager");
      setDatabaseManager(manager);
    };

    initializeDatabaseManager();
  }, []);

  async function logIn2() {
    if (databaseManager) {
      const loginSuccess = await invoke("log_in", { username, password, manager: databaseManager });
      setIsLoggedIn(loginSuccess);
    }
  }

  async function logIn(){
    const loginSuccess = await invoke("test", { })
    setIsLoggedIn(loginSuccess);
  }

  // Placeholder functions for demonstration
  async function functionOne() {
    console.log("Function One called");
    // Implement your logic here
  }

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

          <form className="row" onSubmit={(e) => {
              e.preventDefault();
              logIn();
          }}>
              <div>
                  <input
                      id="username-input"
                      onChange={(e) => setUsername(e.currentTarget.value)}
                      placeholder="Username"
                  />
              </div>
              <div>
                  <input
                      id="password-input"
                      onChange={(e) => setPassword(e.currentTarget.value)}
                      placeholder="Password"
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
