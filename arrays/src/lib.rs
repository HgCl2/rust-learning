pub fn summ(a: [i32; 32]) -> i32 {
    let mut res: i32 = 0;

    for num in a{
        res += num;
    }

    return res;
}

pub fn thirtytwo_tens() -> [i32; 32]{
    return [10; 32];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = summ(thirtytwo_tens());
        assert_eq!(result, 320);
    }
}
