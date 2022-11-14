pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut st = "ABC";
        let result = rev_str(st);
        assert_eq!(result, "CBA");
    }
}
