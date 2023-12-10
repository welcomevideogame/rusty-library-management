import { useState } from "react";
import "./App.css";
import SignIn from "./SignIn";
import Dashboard from "./dashboard/dashboard";

function App() {
  const [isLoggedIn, handleLoginSuccess] = useState(false);

  return (
    <div className="container">
      {!isLoggedIn ? (
        <>
        <SignIn onLoginSuccess={handleLoginSuccess} />
        </>
      ) : (
        <>
          <Dashboard onLogOut={handleLoginSuccess} />
        </>
      )}
    </div>
  );
}

export default App;
