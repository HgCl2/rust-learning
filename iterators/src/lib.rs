#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v % 2 == 0{
            self.v /= 2;
        }else {
            self.v = self.v * 3 + 1;
        }

        return Some(self.v);
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
        collatz_strt.next();
    }

    return counter;
}
