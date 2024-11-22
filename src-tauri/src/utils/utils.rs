use std::{error::Error, fs, net::IpAddr};

use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, types::Bounds, Browser, LaunchOptionsBuilder};
use sanitize_filename::sanitize;

pub fn read_wordlist(dir: &str) -> Result<Vec<String>, &str> {
    match fs::read_to_string(dir) {
        Ok(content) => {
            let wordlist: Vec<String> = content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line: &str| line.to_string())
                .collect();
            Ok(wordlist)
        }
        Err(_) => Err("Failed to load a file!"),
    }
}

pub fn construct_payload<'a>(target_ip: IpAddr, target_port: &str, word: &'a str) -> Result<String, &'a str> {
    if target_ip.is_ipv4() || target_ip.is_unspecified() {
        Ok(format!("http://{}:{}/{}", target_ip, target_port, word))
    } else {
        Err("Not valid ipv4 address!")
    }
}