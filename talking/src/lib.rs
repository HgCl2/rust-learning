pub fn talking(text: &str) -> &str {
    eprintln!("{}", text);
    let trimmed_text = text.trim();
    if trimmed_text == ""{
        return "Just say something!";
    }

    let last_char = trimmed_text.chars().nth(trimmed_text.len()-1).unwrap();

    if trimmed_text == trimmed_text.to_uppercase() && 
    trimmed_text.len() != 1{
        if last_char == '?'{
            return "Quiet, I am thinking!";
        }

        return "There is no need to yell, calm down!";
    }else if last_char == '?'{
        return "Sure.";
    }

    return "Interesting";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
