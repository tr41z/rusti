use std::fs;

pub fn read_wordlist(dir: &str) -> Vec<String> {
    let mut wordlist: Vec<String> = Vec::new();
    let ctx = fs::read_to_string(dir)
        .expect("Failed to read file!");

    wordlist.push(ctx);
    return wordlist;
}