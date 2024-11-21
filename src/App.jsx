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
    <main>
      <div className="options-container">
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            scan();
          }}
        >
          <input
            id="targetIp-input"
            className="input"
            onChange={(e) => setTargetIp(e.currentTarget.value)}
            placeholder="Enter target IP"
          />
          <input
            id="wordlistPath-input"
            className="input"
            onChange={(e) => setWordlistPath(e.currentTarget.value)}
            placeholder="Enter wordlist path"
          />
          <button type="submit" className="submit-button">
            Scan
          </button>
        </form>
      </div>
      <div className="results-container">
        {Object.entries(scanResults)
          .sort(([endpointA, codeA], [endpointB, codeB]) => {
            // First sort by status code categories (200, 300, 400)
            const categoryA = Math.floor(codeA / 100);
            const categoryB = Math.floor(codeB / 100);
            if (categoryA !== categoryB) {
              return categoryA - categoryB; // sort by category
            }
            // If in the same category, sort by status code value
            if (codeA !== codeB) {
              return codeA - codeB;
            }
            // Finally, sort alphabetically by endpoint
            return endpointA.localeCompare(endpointB);
          })
          .map(([endpoint, code]) => {
            // Determine the color dynamically
            let codeColor;
            if (code === 200) {
              codeColor = "green";
            } else if ([403, 401, 301, 302, 307].includes(code)) {
              codeColor = "yellow";
            } else {
              codeColor = "red";
            }

            return (
              <button className="results-elements" key={endpoint}>
                <p>{endpoint}</p>
                <p style={{ color: codeColor }}>{code}</p>{" "}
              </button>
            );
          })}
      </div>
      <div className="preview">
        <img src="public/test.png" width={530} height={650} alt="preview-img" />
      </div>
    </main>
  );
}

export default App;
