pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let quotient = x / y;
    let remainder;
    if x < y{
        remainder = x;
    } else if x == y {
        remainder = 0;
    } else {
        remainder = x - quotient * y;
    }

    return (quotient, remainder);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = 9;
        let y = 4;
        let (division, remainder) = divide(x, y);
        assert_eq!((division, remainder), (2, 1));
    }
}
