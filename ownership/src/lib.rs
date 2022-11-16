pub fn first_subword(mut s: String) -> String {
    let mut result_str = String::new();
    let mut ind = 0;

    for letter in s.chars(){
        if letter >= 'a' && letter <= 'z' { 
            result_str.push(letter);
            ind += 1;
        }else if letter >= 'A' && letter <= 'Z' && ind == 0{
            result_str.push(letter);
            ind += 1;
        }else{
            break;
        }

    }

    return result_str.clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let s = String::from("helloWorld");
        let result = first_subword(s);
        assert_eq!(result, "hello".to_string());
    }
    #[test]
    fn case1() {
        let s = String::from("HelloWorld");
        let result = first_subword(s);
        assert_eq!(result, "Hello".to_string());
    }
    #[test]
    fn case2() {
        let s = String::from("hello_world");
        let result = first_subword(s);
        assert_eq!(result, "hello".to_string());
    }
    #[test]
    fn case3() {
        let s = String::from("just");
        let result = first_subword(s);
        assert_eq!(result, "just".to_string());
    }
}
