pub fn factorial(num: u64) -> u64 {
    let mut result: u64 = 1;
    let mut ind = 1;
    loop {
        if ind > num {
            return result;
        } else {
            result *= ind;
            ind += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(19);
        assert_eq!(result, 121645100408832000);
    }
}
