use std::{fs, net::{IpAddr, TcpStream}, time::Duration};

// Function to retrieve words in each line in wordlist and return them in Vector
pub fn read_wordlist(dir: &str) -> Result<Vec<String>, &str> {
    match fs::read_to_string(dir) {
        Ok(content) => {
            let wordlist: Vec<String> = content
                .lines() // iterates over lines
                .filter(|line| !line.trim().is_empty()) // trimming whitespaces
                .map(|line: &str| line.to_string()) // converts line to &str
                .collect();
            Ok(wordlist)
        }
        Err(_) => Err("Failed to load a file!"),
    }
}

// Function to get ip, port, word and construct full request ready payload to be sent
pub fn construct_payload<'a>(target_ip: IpAddr, target_port: &str, word: &'a str) -> Result<String, &'a str> {
    if !target_ip.is_ipv4() && !target_ip.is_unspecified() {
        return Err("Not valid ipv4 address!");
    }

    match target_port.parse::<u16>() {
        Ok(_) => Ok(format!("http://{}:{}/{}", target_ip, target_port, word)),
        Err(_) => Err("Not valid port number!"),
    }
}

// Validates if the host is reachable by attempting a TCP connection
pub fn is_host_reachable(target_ip: IpAddr, target_port: &str) -> bool {
    let address = format!("{}:{}", target_ip, target_port);
    TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(3)).is_ok()
}