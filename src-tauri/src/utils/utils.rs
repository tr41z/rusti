use std::fs;

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

pub fn construct_payload(target_ip: &String, word: &String) -> String {
    format!("http://{}:80/{}", target_ip, word) // NOTE: 8080 is a placeholder for now, then give user an option to input port_no
}