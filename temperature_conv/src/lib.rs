pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - 32.0) / (9.0/5.0); 
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    return c * (9.0 / 5.0) + 32.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let result = fahrenheit_to_celsius(20.0);
        assert_eq!(result, -6.666666666666666);
    }

    #[test]
    fn case1() {
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);
    }
}
