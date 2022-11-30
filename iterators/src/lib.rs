#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 || self.v == 1{
            return None;
        }
        if self.v % 2 == 0{
            self.v = self.v / 2;
            return Some(self.v * 2);
        }else {
            self.v = self.v * 3 + 1;
            return Some((self.v - 1) / 3);
        }

       
    }
    
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    println!("{}", n);
    if n == 0{
        return 0;
    }

    let mut counter = 0;
    let mut collatz_strt = Collatz::new(n);

    return collatz_strt.count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
