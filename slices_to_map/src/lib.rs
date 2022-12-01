use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T: Eq + Hash, U>(first_slice: &'a [T], second_slice: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut length: usize = 0;
    let length_first = first_slice.len();
    let length_second = second_slice.len();
    
    let mut result = HashMap::new();

    
    if length_first == length_second{
        length = length_first;
    } else if length_first > length_second {
        length = length_second;
    } else if length_first < length_second {
        length = length_first;
    }

    
    for ind in 0..length{
        result.insert(&first_slice[ind], &second_slice[ind]);
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
