pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for ch in input.chars(){
        let new_char: char;
        if key >= 0{
            new_char = (ch as u8 + key as u8) as char;
        }else{
            new_char = (ch as u8 - key.abs() as u8) as char;
        }

        if ch.is_alphabetic() && ch.is_lowercase(){
            if new_char < 'a'{
                result.push(('z' as u8 - 'a' as u8 + new_char as u8 + 1) as char);
            }else if new_char > 'z' {
                result.push(('a' as u8 + new_char as u8 - 'z' as u8 - 1) as char);
            }else{
                result.push(new_char);
            }
        }else if ch.is_alphabetic() && ch.is_uppercase(){
            if new_char < 'A'{
                result.push(('Z' as u8 - 'A' as u8 + new_char as u8 + 1) as char);
            }else if new_char > 'Z'{
                result.push(('A' as u8 + new_char as u8 - 'Z' as u8 - 1) as char);
            }else{
                result.push(new_char);
            }
        }else{
            result.push(ch);
        }

    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rotate("a", 26), "a".to_string());
        assert_eq!(rotate("m", 0), "m".to_string());
        assert_eq!(rotate("a", -1), "z".to_string());
        assert_eq!(rotate("testing", -14), "fqefuzs".to_string());
    }
}
