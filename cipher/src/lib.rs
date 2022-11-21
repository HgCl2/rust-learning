use std::u32;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation: validation,
            expected: expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let mut right_cipher = String::new();

    if original.trim().is_empty() {
        return Option::None;
    }

    for ch in original.chars(){
        if ch.is_alphabetic() && ch.is_lowercase() {
            let new_letter = char::from_u32(((u32::from(ch) - 
            u32::from('a')) * 25 + 25 ) % 26 + u32::from('a')).unwrap();
            right_cipher.push(new_letter);
        }else if ch.is_alphabetic(){
            let new_letter = char::from_u32(((u32::from(ch) - 
            u32::from('A')) * 25 + 25 ) % 26 + u32::from('A')).unwrap();
            right_cipher.push(new_letter);
        }else {
            right_cipher.push(ch);
        };
    }

    if right_cipher != ciphered {
        return Some(Err(CipherError { validation: (false), expected: (right_cipher) }));
    }

    return Some(Result::Ok(true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = cipher("1Hello 2world!", "1Svool 2dliow!");
        assert_eq!(res, Some(Ok(true)));

        let res = cipher("1Hello 2world!", "svool");
        assert_eq!(res, Some(Err(CipherError { validation: false, expected: "1Svool 2dliow!".to_string() })));

        let res = cipher("", "svool");
        assert_eq!(res, Option::None);
    }
}
