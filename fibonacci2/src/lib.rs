pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1{
        return n;
    }
        
    let mut counter = n;
    let mut prevN = 0;
    let mut nextN = 1;
    let mut temp = 0;

    loop {
        if counter == 1{
            return nextN;
        }

        temp = nextN;
        nextN += prevN;
        prevN = temp;
        counter -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let result = fibonacci(0);
        assert_eq!(result, 0);
    }
    
    #[test]
    fn case1() {
        let result = fibonacci(2);
        assert_eq!(result, 1);
    }
    
    #[test]
    fn case2() {
        let result = fibonacci(4);
        assert_eq!(result, 3);
    }
    
    #[test]
    fn case3() {
        let result = fibonacci(22);
        assert_eq!(result, 17711);
    }
    
    #[test]
    fn case4() {
        let result = fibonacci(20);
        assert_eq!(result, 6765);
    }
}
