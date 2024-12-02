use std::{collections::HashMap, net::IpAddr};

use tauri::AppHandle;

pub fn start_enumerator(
    target_ip: IpAddr,
    target_port: &str,
    wordlist_path: &str,
    app_handle: AppHandle,
) {
    // HashMap for storing enumeration results
    let mut enum_results: HashMap<String, u16> = HashMap::new();
}
