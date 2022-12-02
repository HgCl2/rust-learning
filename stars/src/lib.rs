pub fn stars(n: u32) -> String {
    let times = f32::powf(2.0, n as f32) as usize;
    let elem: String = String::from("*");
    let mut result = String::new();
    for _ in 0..times{
        result.push_str(&elem);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
