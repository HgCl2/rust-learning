use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut first_str = HashMap::new();
    let mut second_str = HashMap::new();

    for ch in s1.chars() {
        first_str.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    for ch in s2.chars() {
        second_str.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    return first_str == second_str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
	    let word1 = "thougth";
        assert_eq!(is_permutation(word, word1), true);
    }
}
