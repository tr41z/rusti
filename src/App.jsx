import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [targetIp, setTargetIp] = useState("");
  const [targetPort, setTargetPort] = useState("");
  const [wordlistPath, setWordlistPath] = useState("");
  const [scanResults, setScanResults] = useState([]);
  const [selectedPreview, setSelectedPreview] = useState(null);

  async function scan() {
    await invoke("init_sniffer", { targetIp, targetPort, wordlistPath });
  }

  useEffect(() => {
    // Fetch scan results
    listen("scan_results", (event) => {
      setScanResults(event.payload);
      console.log(event.payload);
    });
  }, []);

  return (
    <main>
      {/* Options form */}
      <div className="options-container">
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            scan();
          }}
        >
          <div className="ip-port">
            <input
              className="input-ip"
              onChange={(e) => setTargetIp(e.currentTarget.value)}
              placeholder="target IP address..."
            />
            <input
              className="input-port"
              onChange={(e) => setTargetPort(e.currentTarget.value)}
              placeholder="port number..."
            />
          </div>
          <input
            className="input"
            onChange={(e) => setWordlistPath(e.currentTarget.value)}
            placeholder="/usr/share/wordlists/..."
          />
          <button type="submit" className="submit-button">
            Scan
          </button>
        </form>
      </div>

      {/* Scan results */}
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
              <button
                className="results-elements"
                key={endpoint}
                onClick={() => setSelectedPreview(endpoint)} // update selected preview
              >
                <p>{endpoint}</p>
                <p style={{ color: codeColor }}>{code}</p>
              </button>
            );
          })}
      </div>

      {/* Selected preview */}
      <div className="preview">
        {selectedPreview ? (
          <iframe
            className="preview-iframe"
            src={selectedPreview}
            title="Selected Preview"
          />
        ) : (
          <p>Select an endpoint to preview</p>
        )}
      </div>
    </main>
  );
}

export default App;
