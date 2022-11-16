pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    return (c, (c as f64).exp(), (c as f64).ln());
}

pub fn str_function(a: String) -> (String, String) {
    let split = a.split_whitespace();
    let mut result = String::new();
    for num in split{
        let num: f64 = num.parse().expect("not a number");
        result.push_str(&(num.exp().to_string() + &String::from(" ")));
    }

    let length = result.len()-1;
    result = result[0..length].to_string();
    return (a.clone(), result.clone()); 
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut vector: Vec<f64> = Vec::new();
    let iter = b.clone();

    for elem in iter{
        vector.push((elem as f64).ln());
    }

    return (b, vector);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_nbr_function() {
        let a: i32 = 0;
        let result = nbr_function(a);
        let right = (0, 1.0, -f64::INFINITY);
        assert_eq!(result, right);
    }

    #[test]
    fn test_str_function() {
        let a = String::from("1 2 4 5 6");
        let result = str_function(a);
        let right = ("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string());
        assert_eq!(result, right);
    }

    #[test]
    fn test_vec_function() {
        let c = vec![1, 2, 4, 5];
        let result = vec_function(c);
        let right = (vec![1, 2, 4, 5], vec![0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]);
        assert_eq!(result, right);
    }
}
