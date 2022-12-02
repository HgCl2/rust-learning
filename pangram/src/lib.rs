pub fn is_pangram(s: &str) -> bool {
    let mut counter = [0; 26];

    for ch in s.chars(){
        match ch {
            'a'..='z' => counter[ch as usize - 'a' as usize] += 1,
            'A'..='Z' => counter[ch as usize - 'A' as usize] += 1,
            _ => continue,
        }
    }

    for elem in counter{
        if elem == 0{
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
