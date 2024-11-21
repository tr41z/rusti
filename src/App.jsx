import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import ScanResults from "./views/ScanResults";

function App() {
  const [targetIp, setTargetIp] = useState("");
  const [wordlistPath, setWordlistPath] = useState("");

  async function scan() {
    await invoke("init_sniffer", { targetIp, wordlistPath });
  }

  return (
    <main className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          scan();
        }}
      >
        <input
          id="targetIp-input"
          onChange={(e) => setTargetIp(e.currentTarget.value)}
          placeholder="Enter target IP"
        />
        <input
          id="wordlistPath-input"
          onChange={(e) => setWordlistPath(e.currentTarget.value)}
          placeholder="Enter wordlist path"
        />
        <button type="submit">Scan</button>
      </form>
      <ScanResults />
    </main>
  );
}

export default App;
