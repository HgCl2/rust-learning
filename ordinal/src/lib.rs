pub fn num_to_ordinal(x: u32) -> String {
    let mut result = x.to_string();

    let last_digit = x % 10;

    if last_digit == 1{
        result.push_str("st");
    }else if last_digit == 2{
        result.push_str("nd");
    }else if last_digit == 3{
        result.push_str("rd");
    }else {
        result.push_str("th");
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
