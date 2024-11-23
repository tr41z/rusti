use std::{collections::HashMap, net::IpAddr};

use crate::utils::utils::{self, construct_payload};

use reqwest::blocking;
use tauri::{AppHandle, Emitter};

pub fn start_scanner(target_ip: IpAddr, target_port: &str, wordlist_path: &str, app_handle: AppHandle) {
    let mut scan_results: HashMap<String, u16> = HashMap::new(); // HashMap for storing scan results (endpoint, code)

    match utils::read_wordlist(&wordlist_path) { // reading provided wordlist
        Ok(wordlist) => {
            for word in wordlist.iter() {
                // Constructing payloads from provided ip, port and word in wordlist
                let payload: String = construct_payload(target_ip, target_port, word).unwrap();
                match blocking::get(&payload) { // sending request to each endpoint constructed in previous step
                    Ok(res) => {
                        scan_results.insert(payload.clone(), res.status().as_u16()); // insterting results
                    }
                    Err(_) => {
                        eprintln!("Failed to send request. Wrong IP address");
                    }
                }
            }
            // Emitting all scan results to capture in front-end
            app_handle.emit("scan_results", scan_results).unwrap_or_else(|e| {
                eprintln!("Failed to emit scan results: {}", e);
            });
        }
        Err(_) => {
            eprintln!("Failed to read wordlist");
        }
    }
}