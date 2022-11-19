pub fn sum(a: &[i32]) -> i32 {
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
        let result = sum(&thirtytwo_tens());
        assert_eq!(result, 320);
    }

    #[test]
    fn case2() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let a1: Vec<i32> = (1..11).collect();
        let b = [5; 10];
        assert_eq!(sum(&a), 55);
        assert_eq!(sum(&a1), 55);
        assert_eq!(sum(&b), 50);


    }
}
