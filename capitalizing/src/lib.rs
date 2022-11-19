pub fn capitalize_first(input: &str) -> String {
    let mut res: String = String::new();

    for (i, ch) in input.chars().enumerate() {
        if i == 0 && ch.is_lowercase() {
            res.push(('A' as u8 + ch as u8 - 'a' as u8) as char);
        }else{
            res.push(ch)
        }
    }

    return res;
}

pub fn title_case(input: &str) -> String {
    let mut res: String = String::new();

    let mut word_started = true;
    for ch in input.chars(){
        if word_started && ch.is_lowercase() {
            res.push(('A' as u8 + ch as u8 - 'a' as u8) as char);
            word_started = false;
        }else if ch.is_whitespace(){
            word_started = true;
            res.push(ch);
        }else{
            res.push(ch)
        }
    }

    return res;
}

pub fn change_case(input: &str) -> String {
    let mut res: String = String::new();

    for ch in input.chars(){
        if ch.is_alphabetic() && ch.is_lowercase() {
            res.push(('A' as u8 + ch as u8 - 'a' as u8) as char);
        }else if ch.is_alphabetic() && ch.is_uppercase() {
            res.push(('a' as u8 + ch as u8 - 'A' as u8) as char);
        }else {
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
        assert_eq!(capitalize_first("joe is missing"), "Joe is missing".to_string());
        assert_eq!(title_case("jill is leaving A"), "Jill Is Leaving A".to_string());
        assert_eq!(change_case("heLLo THere"), "HEllO thERE".to_string());
    }
}
