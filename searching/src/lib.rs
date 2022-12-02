pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (ind, value) in array.iter().enumerate(){
        if *value == key{
            return Some(ind);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ar = [1, 3, 4, 6, 8, 9, 11];
        let f = search(&ar, 6);
        assert_eq!(f, Some(3));
    }
}
