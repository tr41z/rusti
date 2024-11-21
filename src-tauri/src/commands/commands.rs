use std::net::IpAddr;

use crate::probing::{enumerator, scanner};

pub fn start_enumerator() {enumerator::start_enumerator();}

pub fn start_scanner(target_ip: IpAddr, wordlist_path: String) {
    scanner::start_scanner(target_ip, wordlist_path);
}