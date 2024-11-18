use crate::utils::utils;

pub fn start_scanner() {
    let wordlist = utils::read_wordlist("/Users/michael/Desktop/Coding/Rust/rusti/src-tauri/src/probing/test.txt");
    for word in wordlist.iter() {
        println!("{}", word);
    }
}