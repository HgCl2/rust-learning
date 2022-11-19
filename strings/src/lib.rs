pub fn char_length(s: &str) -> usize {
    return s.chars().count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = char_length("❤");
        assert_eq!(result, 1);
        let result = char_length("形声字");
        assert_eq!(result, 3);
        let result = char_length("change");
        assert_eq!(result, 6);
        let result = char_length("😍");
        assert_eq!(result, 1);
    }
}
