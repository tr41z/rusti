#[cfg(test)]
mod tests {
    use std::fs;

    use crate::utils::utils::read_wordlist;

    #[test]
    fn test_read_wordlist_success() {
        let test_file: &str = "test_wordlist.txt";
        let content: &str = "images\nimages/favicon.ico\nword3";

        fs::write(test_file, content).unwrap();
        let result: Result<Vec<String>, &str> = read_wordlist(test_file);
        fs::remove_file(test_file).unwrap();

        assert!(result.is_ok());
        let wordlist: Vec<String> = result.unwrap();
        
        assert_eq!(wordlist, vec!["images", "images/favicon.ico", "word3"]);
    }

    #[test]
    fn test_read_wordlist_whitespaces() {
        let test_file: &str = "test_wordlist.txt";
        let content: &str = "images\n\n\n\nimages/favicon.ico\n\n\n\nword3";

        fs::write(test_file, content).unwrap();
        let result: Result<Vec<String>, &str> = read_wordlist(test_file);
        fs::remove_file(test_file).unwrap();

        assert!(result.is_ok());
        let wordlist: Vec<String> = result.unwrap();
        
        assert_eq!(wordlist, vec!["images", "images/favicon.ico", "word3"]);
    }

    #[test]
    fn read_wordlist_failure_404() {
        let result: Result<Vec<String>, &str> = read_wordlist("nullfile.txt");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Failed to load a file!"));
    }
}