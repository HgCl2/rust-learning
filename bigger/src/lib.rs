use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max: i32 = -65000;
    for (key, val) in h.iter() {
        if *val > max {
            max = *val;
        }
    }

    return max;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut hash = HashMap::new();
        hash.insert("Daniel", 122);
        hash.insert("Ashley", 333);
        hash.insert("Katie", 334);
        hash.insert("Robert", 14);
        assert_eq!(bigger(hash), 334);
    }
}
