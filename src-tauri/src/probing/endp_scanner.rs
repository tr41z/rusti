use std::{collections::HashMap, net::IpAddr};

use crate::utils::utilities::{self, construct_payload, is_host_reachable};

use reqwest::blocking;
use tauri::{AppHandle, Emitter};

pub fn start_scanner(
    target_ip: IpAddr,
    target_port: &str,
    wordlist_path: &str,
    app_handle: AppHandle,
) {
    let mut scan_results: HashMap<String, u16> = HashMap::new(); // HashMap for storing scan results (endpoint, code)
    let _client: reqwest::Client = reqwest::Client::new(); // PERF: implement header input processing

    // Validate if the target host is reachable
    if !is_host_reachable(target_ip, target_port) {
        eprintln!("Host is unreachable: {}:{}", target_ip, target_port);
        app_handle
            .emit("scan_error", "Host is unreachable")
            .unwrap_or_else(|e| {
                eprintln!("Failed to emit scan error: {}", e);
            });
        return;
    }

    // Reading provided wordlist
    match utilities::read_wordlist(wordlist_path) {
        Ok(wordlist) => {
            for word in wordlist.iter() {
                // Constructing payloads from provided ip, port, and word in wordlist
                let payload = match construct_payload(target_ip, target_port, word) {
                    Ok(url) => url,
                    Err(_) => {
                        eprintln!("Failed to construct payload for word: {}", word);
                        continue;
                    }
                };

                // Send the request
                match blocking::get(&payload) {
                    Ok(res) => {
                        // Inserting results
                        scan_results.insert(payload.clone(), res.status().as_u16());
                    }
                    Err(err) => {
                        eprintln!("Failed to send request to {}: {}", payload, err);
                    }
                }
            }

            // Emit all scan results to capture in the front-end
            app_handle
                .emit("scan_results", scan_results)
                .unwrap_or_else(|e| {
                    eprintln!("Failed to emit scan results: {}", e);
                });
        }
        Err(err) => {
            eprintln!("Failed to read wordlist: {}", err);
            app_handle.emit("scan_error", err).unwrap_or_else(|e| {
                eprintln!("Failed to emit scan error: {}", e);
            });
        }
    }
}
