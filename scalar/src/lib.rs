pub fn summ(a: u8, b: u8) -> u8 {
    return a + b;
}

pub fn diff(a: i16, b: i16) -> i16 {
    return a - b;
}

pub fn pro(a: i8, b: i8) -> i8 {
    return a * b;
}

pub fn quo(a: f32, b: f32) -> f32 {
    return a/b;
}

pub fn rem(a: f32, b: f32) -> f32 {
    let int_a: i32 = a as i32;
    let int_b: i32 = b as i32;
   
    if int_a / int_b == 0{
        return 0.0;
    }

    return a - ((int_a / int_b) * int_b) as f32;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn case0() {
        let result = rem(2.0, 5.0);
        assert_eq!(result, 0.0);
    }
    #[test]
    fn case1() {
        let result = summ(1, 255);
        assert_eq!(result, 0);
    }
    #[test]
    fn case2() {
        let result = diff(-32768, 32766);
        assert_eq!(result, 0);
    }
    #[test]
    fn case3() {
        let result = pro(-128, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn case4() {
        let result = rem(-128.23, 2.0);
        assert_eq!(result, -0.22999573);
    }

}
