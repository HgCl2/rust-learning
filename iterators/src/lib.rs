#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == u64::MAX || self.v == u64::MIN+1{
            return None;
        }else {
            self.v -= 1;
            return Some(*self);
        }
    }
    
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0{
        return 0;
    }

    let mut counter = 0;
    let mut collatz_strt = Collatz::new(n);
    while collatz_strt.v != 1 {
        counter += 1;
        if collatz_strt.v % 2 == 0{
            collatz_strt.v = collatz_strt.v / 2;
        }else {
            collatz_strt.v = collatz_strt.v * 3 + 1;
        }
    }

    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
