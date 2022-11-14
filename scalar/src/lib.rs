pub fn sum(a: u8, b: u8) -> u8 {
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
   
    if int_a < int_b{
        return a;
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


    fn case3() {
        let result = rem(-128, 2);
        assert_eq!(result, 0);
    }

}
