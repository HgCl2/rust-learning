pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A'{
        return vec!["A".to_string()];
    }

    let mut result = Vec::<String>::new();
    //let distance = c as u8 - 'A' as u8;

    let mut space_after = 0;
    for letter in 'A'..=c{
        let mut elem = String::new();
        let space_before = c as usize - letter as usize;
        
        elem.push_str(&" ".repeat(space_before));
        elem.push(letter);
        

        if letter != 'A'{ 
            space_after += 1;
            elem.push_str(&" ".repeat(space_after));
            elem.push_str(&" ".repeat(space_after - 1));
            elem.push(letter);
        }
        
        elem.push_str(&" ".repeat(space_before));

        result.push(elem);
    }

    for letter in ('A'..c).rev(){
        let mut elem = String::new();
        let space_before = c as usize - letter as usize;
        
        elem.push_str(&" ".repeat(space_before));
        elem.push(letter);
        

        if letter != 'A'{ 
            space_after -= 1;
            elem.push_str(&" ".repeat(space_after));
            elem.push_str(&" ".repeat(space_after - 1));
            elem.push(letter);
        }
        
        elem.push_str(&" ".repeat(space_before));

        result.push(elem);
    }

    return result;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
