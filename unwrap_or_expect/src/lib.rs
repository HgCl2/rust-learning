pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn msg(x: String) -> String {
    let mut res: String = String::new();
    res.push_str("Not found: ");
    res.push_str(&x);

    return res.clone();
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap(),
        Security::High => server.expect("ERROR: program stops"),
        Security::Medium => server.unwrap_or("WARNING: check the server".to_string()),
        Security::Low => server.unwrap_or_else(msg),
        Security::BlockServer => server.unwrap_err(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medium() {
        let result = fetch_data(Ok("server1.com".to_string()), Security::Medium);
        assert_eq!(result, "server1.com");
        let result = fetch_data(Err(String::new()), Security::Medium);
        assert_eq!(result, "WARNING: check the server"); 
    }

    #[test]
    fn low() {
        let result = fetch_data(Err("server2.com".to_string()), Security::Low);
        assert_eq!(result, "Not found: server2.com");
    }

}
