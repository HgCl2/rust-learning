use std::ops::Add;
#[derive(Debug)]
pub struct StepIterator<T: Add<Output = T>> {
    pub begin: T,
    pub end: T,
    pub step: T,
}
    
impl<T: Add<Output = T>> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator { begin: beg, end, step }
    }
}
    
impl<T> std::iter::Iterator for StepIterator<T>
where T: Add<Output = T> + PartialEq<T> + Clone + PartialOrd {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let result: T;

        if self.begin > self.end {
            return None;
        }else{
            result = self.begin.clone();
            self.begin = self.begin.clone() + self.step.clone();
        }

        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
