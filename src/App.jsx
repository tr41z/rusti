import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [targetIp, setTargetIp] = useState("");
  const [wordlistPath, setWordlistPath] = useState("");
  const [scanResults, setScanResults] = useState([]);
  
  async function scan() {
    await invoke("init_sniffer", { targetIp, wordlistPath });
  }

  async function fetchScanResults() {
    await listen("scan_results", (event) => {
      setScanResults(event.payload);
      console.log(event.payload)
    })
  }

  useEffect(() => {
    fetchScanResults();
  }, [])

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
      {Object.entries(scanResults).map(([endp, code]) => {
        return (
          <div>
            <p>{endp}</p>
            <p>{code}</p>
          </div>
        );
      })}
    </main>
  );
}

export default App;
