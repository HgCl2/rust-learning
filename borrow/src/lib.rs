pub fn str_len(s: &str) -> usize {
    return s.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let s = "hello";
        let result = str_len(&s);
        assert_eq!(result, 5);
    }

    #[test]
    fn case1() {
        let s1 = "camelCase".to_string();
        let result = str_len(&s1);
        assert_eq!(result, 9);
    }
}
