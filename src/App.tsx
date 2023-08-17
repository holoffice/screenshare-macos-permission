import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { useState } from "react";

function App() {
  const [message, setMessage] = useState('')

  async function screenshot() {
    setMessage("")
    const message: string = await invoke("screenshot")
    setMessage(message)
  }

  return (
    <div className="container">
      <button onClick={screenshot}>Take Screenshot</button>
      <p>{message}</p>
    </div>
  );
}

export default App;
