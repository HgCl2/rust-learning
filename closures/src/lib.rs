pub fn first_fifty_even_square() -> Vec<i32> {
    let mut result_vec: Vec<i32> = Vec::new();

    let mut cur_num = 2;
    let is_even = |num: i32| -> bool {num % 2 == 0};
    while result_vec.len() != 50 {
        if is_even(cur_num){
            result_vec.push(cur_num * cur_num);
        }
        cur_num += 1;
    }
    return result_vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
