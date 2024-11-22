use std::net::IpAddr;

use tauri::AppHandle;

use crate::probing::{enumerator, scanner};

pub fn start_enumerator() {enumerator::start_enumerator();}

pub fn start_scanner(target_ip: IpAddr, target_port: &str, wordlist_path: String, app_handle: AppHandle) {
    scanner::start_scanner(target_ip, target_port, wordlist_path, app_handle);
}