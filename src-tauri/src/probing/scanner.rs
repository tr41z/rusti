use std::collections::HashMap;

use crate::utils::utils::{self, construct_payload};

use reqwest::{blocking, StatusCode};

pub fn start_scanner(target_ip: String, wordlist_path: String) {
    let mut scan_results: HashMap<String, StatusCode> = HashMap::new();

    match utils::read_wordlist(&wordlist_path) {
        Ok(wordlist) => {
            for word in wordlist.iter() {
                let payload: String = construct_payload(&target_ip, word);
                match blocking::get(&payload) {
                    Ok(res) => {
                        scan_results.insert(payload.clone(), res.status());
                    }
                    Err(_err) => {
                        eprintln!("Failed to send request to {}. Wrong IP address", payload);
                    }
                }
            }
            println!("HashMap with results: {:#?}", scan_results);
        }
        Err(_) => {
            eprintln!("Failed to read wordlist");
        }
    }
}