use std::net::IpAddr;

use tauri::AppHandle;

use crate::probing::{enumerator, scanner};

pub fn start_enumerator() {enumerator::start_enumerator();}

pub fn start_scanner(target_ip: IpAddr, wordlist_path: String, app_handle: AppHandle) {
    scanner::start_scanner(target_ip, wordlist_path, app_handle);
}