use std::{collections::HashMap, net::IpAddr};

use crate::utils::utils::{self, construct_payload};

use reqwest::blocking;
use tauri::{AppHandle, Emitter};

// NOTE: use `headless_chrome = "0.9.0"` for taking screenshots of live endpoints (200, 301, 302 etc..)
pub fn start_scanner(target_ip: IpAddr, wordlist_path: String, app_handle: AppHandle) {
    let mut scan_results: HashMap<String, u16> = HashMap::new();

    match utils::read_wordlist(&wordlist_path) {
        Ok(wordlist) => {
            for word in wordlist.iter() {
                let payload: String = construct_payload(target_ip, word).unwrap();
                match blocking::get(&payload) {
                    Ok(res) => {
                        scan_results.insert(payload.clone(), res.status().as_u16());
                    }
                    Err(_) => {
                        eprintln!("Failed to send request. Wrong IP address");
                    }
                }
            }
            app_handle.emit("scan_results", scan_results).unwrap_or_else(|e| {
                eprintln!("Failed to emit scan results: {}", e);
            });

        }
        Err(_) => {
            eprintln!("Failed to read wordlist");
        }
    }
}