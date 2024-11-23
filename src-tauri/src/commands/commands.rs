use std::net::IpAddr;

use tauri::AppHandle;

use crate::probing::{sub_enumerator, endp_scanner};

pub fn start_enumerator() {sub_enumerator::start_enumerator();}

pub fn start_scanner(target_ip: IpAddr, target_port: &str, wordlist_path: &str, app_handle: AppHandle) {
    endp_scanner::start_scanner(target_ip, target_port, wordlist_path, app_handle);
}