use crate::utils::utils;

pub fn start_scanner(target_ip: String, wordlist_path: String) {
    let wordlist = utils::read_wordlist(&wordlist_path);
    for word in wordlist.iter() {
        println!("{}", word);
    }

    println!("Started scan for: {} with wordlist from path: {}", target_ip, wordlist_path);
}