
pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let split_s = s.split_ascii_whitespace();
    let mut result_vec: Vec<u32> = Vec::new();
    for word in split_s{
        if word.ends_with('k'){
            let new_word = word.replace("k", "");
            let number: f32 = new_word.parse().expect("word is invalid");
            result_vec.push((number * 1000.0) as u32);
        }else{
            let number: u32 = word.parse().unwrap();
            result_vec.push(number);
        }
    }

    return Box::new(result_vec);
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    return *a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
