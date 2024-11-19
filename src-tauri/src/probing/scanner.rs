use crate::utils::utils::{self, construct_payload};

pub fn start_scanner(target_ip: String, wordlist_path: String) {
    match utils::read_wordlist(&wordlist_path) {
        Ok(wordlist) => {
            for word in wordlist.iter() {
                let payload: String = construct_payload(&target_ip, word);
                println!("{}", payload);
            }
            println!("Started scan for: {} with wordlist from path: {}", &target_ip, wordlist_path);
        }
        Err(_) => {
            eprintln!("Failed to read wordlist");
        }
    }
}