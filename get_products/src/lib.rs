pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    for cur_ind in 0..arr.len() {
        let mut product = 1;
        for ind in 0..arr.len() {
            if ind != cur_ind{
                product *= arr[ind];
            }
        }
        result.push(product);
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
