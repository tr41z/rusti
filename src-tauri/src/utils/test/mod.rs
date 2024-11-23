#[cfg(test)]
mod tests {
    use std::{fs, net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpStream}, time::Duration};

    use crate::utils::utils::{construct_payload, read_wordlist};

    #[test]
    fn test_read_wordlist_success() {
        let test_file: &str = "test_wordlist_success.txt";
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
        let test_file: &str = "test_wordlist_whitespace.txt";
        let content: &str = "images\n\n\n\nimages/favicon.ico\n\n\n\nword3";

        fs::write(test_file, content).unwrap();
        let result: Result<Vec<String>, &str> = read_wordlist(test_file);
        fs::remove_file(test_file).unwrap();

        assert!(result.is_ok());
        let wordlist: Vec<String> = result.unwrap();
        
        assert_eq!(wordlist, vec!["images", "images/favicon.ico", "word3"]);
    }

    #[test]
    fn test_read_wordlist_failure_not_found() {
        let result: Result<Vec<String>, &str> = read_wordlist("nullfile.txt");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Failed to load a file!"));
    }

    #[test]
    fn test_construct_payload_success() {
        let target_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let target_port:&str = "80";
        let word = "word1";

        let result = construct_payload(target_ip, target_port, word).unwrap();
        assert_eq!(result, "http://127.0.0.1:80/word1");
    }

    #[test]
    fn test_construct_payload_failed() {
        let target_ip: IpAddr = IpAddr::V6(Ipv6Addr::new(12, 12, 12, 12, 12, 12, 12, 12));
        let target_port:&str = "80";
        let word = "word1";

        let result = construct_payload(target_ip, target_port, word);
        assert_eq!(result, Err("Not valid ipv4 address!"));
    }

    #[test]
    fn is_host_reachable_success() {
        let target_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let target_port = "8080"; // remember to run http server on port 8080 before running test
        let address = format!("{}:{}", target_ip, target_port);
        
        let connection_test: bool = TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(3)).is_ok();
        assert_eq!(connection_test, true);
    }

    #[test]
    fn is_host_reachable_failed() {
        let target_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)); // not valid address
        let target_port = "8080";
        let address = format!("{}:{}", target_ip, target_port);
        
        let connection_test: bool = TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(3)).is_ok();
        assert_eq!(connection_test, false);
    }
}