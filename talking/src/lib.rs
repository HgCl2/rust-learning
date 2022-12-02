pub fn talking(text: &str) -> &str {
    if text == ""{
        return "Just say something!";
    }
    
    let last_char = text.chars().nth(text.len()-1).unwrap();

    if text == text.to_uppercase(){
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
