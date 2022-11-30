#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl <'a>Numbers <'_>{
    pub fn new(arr: &'a [u32]) -> Numbers<'a> {
        Numbers { numbers: arr,}
    }

    pub fn list(&self) -> &[u32] {
        return self.numbers;
    }

    pub fn latest(&self) -> Option<u32> {
        let len = self.numbers.len();
        if len == 0{
            return None;
        }

        return Some(self.numbers[len-1]);
    }

    pub fn highest(&self) -> Option<u32> {
        let mut max = 0;

        if self.numbers.len() == 0{
            return None;
        }

        for num in self.numbers{
            if num > &max {
                max = *num;
            }
        }

        return Some(max);
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut result: Vec<u32> = vec![0, 0, 0];

        for num in self.numbers{
            if *num > result[0]{
                result[2] = result[1];
                result[1] = result[0];
                result[0] = *num;
            }else if *num > result[1] {
                result[2] = result[1];
                result[1] = *num;
            }else if *num > result[2] {
                result[2] = *num;
            }
        }

        while !result.is_empty() && result[result.len() - 1] == 0{
            result.pop();
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
