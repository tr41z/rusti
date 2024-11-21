import { listen } from "@tauri-apps/api/event";
import React, { useEffect, useState } from "react";

const ScanResults = () => {
  const [scanResults, setScanResults] = useState([]);

  useEffect(() => {
    let unlisten;
    const setupListener = async () => {
      unlisten = await listen("scan_results", (event) => {
        setScanResults(event.payload);
      });
    };
    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  return (
    <div>
      <p>{JSON.stringify(scanResults)}</p>
    </div>
  );
};

export default ScanResults;
