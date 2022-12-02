pub fn score(msg: &str) -> u64 {
    let mut result: u64 = 0;
    for ch in msg.to_lowercase().chars(){
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' =>
                result += 1,
            'd' | 'g' => result += 2,
            'b' | 'c' | 'm' | 'p' => result += 3,
            'f' | 'h' | 'v' | 'w' | 'y' => result += 4,
            'k' => result += 5,
            'j' | 'x' => result += 8,
            'q' | 'z' => result += 10,
            _ => continue,
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(score("a"), 1);
        assert_eq!(score("ThiS is A Test"), 14);
    }
}
