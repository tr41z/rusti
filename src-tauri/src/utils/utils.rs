use std::{fs, net::IpAddr};

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

pub fn construct_payload<'a>(target_ip: IpAddr, word: &'a str) -> Result<String, &'a str> {
    if target_ip.is_ipv4() || target_ip.is_unspecified() {
        Ok(format!("http://{}:8080/{}", target_ip, word)) // NOTE: 8080 is a placeholder for now, then give user an option to input port_no
    } else {
        Err("Not valid ipv4 address!")
    }
}

#[allow(unused)]
pub fn take_screenshot(payload: String) {}