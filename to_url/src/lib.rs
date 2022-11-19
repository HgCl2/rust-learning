pub fn to_url(s: &str) -> String {
    let mut res: String = String::new();

    for ch in s.chars(){
        if ch.is_whitespace(){
            res.push_str("%20");
        }else{
            res.push(ch);
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        let result = to_url(s);
        assert_eq!(result, "Hello,%20world!".to_string());
    }
}
