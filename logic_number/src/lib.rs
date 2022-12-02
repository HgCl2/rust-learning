pub fn number_logic(num: u32) -> bool {
    let mut sum = 0;
    let digits_count = count_digits(num);
    let mut num_twin = num;

    while num_twin != 0 {
        let digit = num_twin % 10;
        sum += digit.pow(digits_count as u32);
        num_twin /= 10;
    }

    return sum == num;
}

pub fn count_digits(mut num: u32) -> usize {
    let mut result: usize = 0;
    while num != 0{
        result += 1;
        num /= 10;
    }

    return  result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = count_digits(1234);
        assert_eq!(result, 4);
    }
}
