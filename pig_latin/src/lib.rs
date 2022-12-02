const VOWELS: [char; 12] = ['A', 'a', 'E', 'e', 
    'O', 'o', 'I', 'i', 
    'U', 'u', 'Y', 'y'];

const CONSONANTS: [char; 40] = ['Q', 'q', 'W', 'w', 
    'R', 'r', 'T', 't', 'P', 'p', 
    'S', 's', 'D', 'd', 'F', 'f', 
    'G', 'g', 'H', 'h', 'J', 'j', 
    'K', 'k', 'L', 'l', 'M', 'm',
    'N', 'n', 'B', 'b', 'V', 'v', 
    'C', 'c', 'X', 'x', 'Z', 'z'];


pub fn pig_latin(text: &str) -> String {
    let mut result = String::new();
    let first_letter = text.chars().nth(0).unwrap();
    
    if is_vowel(first_letter){
        result.push_str(text);
        result.push_str("ay");
    }else if is_consonant(first_letter){
        if text[1..=2] == *"qu"{
            result.push_str(&text[3..]);
            result.push_str(&text[0..3]);
            result.push_str("ay");
            return result;
        }

        let mut ind = 1;
        let mut cur_letter = text.chars().nth(ind).unwrap();
        while is_consonant(cur_letter){
            ind += 1;
            cur_letter = text.chars().nth(ind).unwrap();
        }

        result.push_str(&text[ind..]);
        result.push_str(&text[..ind]);
        result.push_str("ay");
    }

    return result;
}

pub fn is_consonant(letter: char) -> bool{
    for ch in CONSONANTS{
        if ch == letter{
            return true;
        }
    }

    return false;
}

pub fn is_vowel(letter: char) -> bool{
    for ch in VOWELS{
        if ch == letter{
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
