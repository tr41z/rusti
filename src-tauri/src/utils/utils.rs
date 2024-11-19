use std::fs;

pub fn read_wordlist(dir: &str) -> Result<Vec<String>, &str> {
    let mut wordlist: Vec<String> = Vec::new();
    match fs::read_to_string(dir) {
        Ok(content) => { 
            wordlist.push(content); 
            return Ok(wordlist);
        }
        Err(_) => { return Err("Failed to load a file!") }
    }
}