pub fn talking(text: &str) -> &str {
    let trimmed_text = text.trim();
    if trimmed_text == ""{
        return "Just say something!";
    }

    let last_char = trimmed_text.chars().nth(trimmed_text.len()-1).unwrap();

    if trimmed_text == trimmed_text.to_uppercase() && 
    trimmed_text.len() != 1{
        if is_numeric(&trimmed_text[0..trimmed_text.len()-1]){
            return "Sure.";
        }
        if last_char == '?'{
            return "Quiet, I am thinking!";
        }

        return "There is no need to yell, calm down!";
    }else if last_char == '?'{
        return "Sure.";
    }

    return "Interesting";
}

pub fn is_numeric(text: &str) -> bool{
    for ch in text.chars(){
        if !ch.is_numeric(){
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
