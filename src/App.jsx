import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import toast from "react-hot-toast";
import { isIPv4 } from "is-ip";

function App() {
  const [targetIp, setTargetIp] = useState("");
  const [targetPort, setTargetPort] = useState("");
  const [wordlistPath, setWordlistPath] = useState("");
  const [scanResults, setScanResults] = useState([]);
  const [selectedPreview, setSelectedPreview] = useState(null);
  const [scanError, setScanError] = useState("");
  const [lastError, setLastError] = useState("");

  async function scan() {
    setScanError("");

    // Validate inputs
    if (
      targetIp.trim() === "" ||
      targetPort.trim() === "" ||
      wordlistPath.trim() === ""
    ) {
      toast.error("Input fields must not be empty!");
      return;
    }
    if (!isIPv4(targetIp)) {
      toast.error("IP must be a valid IPv4 address!");
      return;
    }
    if (isNaN(targetPort)) {
      toast.error("Port must be in number format!");
      return;
    }

    // Invoke the scan function
    try {
      await invoke("init_sniffer", { targetIp, targetPort, wordlistPath });
    } catch (error) {
      setScanError(
        error.message || "An error occurred while starting the scan."
      );
    }
  }

  useEffect(() => {
    // Listen for scan results
    const unlistenResults = listen("scan_results", (event) => {
      setScanResults(event.payload);
      console.log("Scan results:", event.payload);
    });

    // Listen for scan errors
    const unlistenErrors = listen("scan_error", (event) => {
      setScanError(event.payload);
    });

    // Cleanup listeners on unmount
    return () => {
      unlistenResults.then((unsub) => unsub());
      unlistenErrors.then((unsub) => unsub());
    };
  }, []);

  useEffect(() => {
    // Display a toast whenever scanError changes
    if (scanError) {
      toast.error(scanError);
      setLastError(scanError); // keep the last error for tracking if needed
    }
  }, [scanError]);

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
              placeholder="127.0.0.1"
            />
            <input
              className="input-port"
              onChange={(e) => setTargetPort(e.currentTarget.value)}
              placeholder="80"
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
        {scanResults.length === 0 ? (
          <p className="no-endpoint">Your endpoints will appear here</p>
        ) : (
          Object.entries(scanResults)
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
            })
        )}
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
