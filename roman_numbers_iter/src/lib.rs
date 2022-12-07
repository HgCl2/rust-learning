
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self{
        match num{
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self{
        let mut result = RomanNumber(Vec::new());
        if num == 0 {
            result.0.push(RomanDigit::from(0));
            return result;
        }

        while num > 0{
            if num > 1000{
                result.0.push(RomanDigit::M);
                num -= 1000;
            }else if num > 900{
                result.0.push(RomanDigit::C);
                result.0.push(RomanDigit::M);
                num -= 900;
            }else if num > 500{
                result.0.push(RomanDigit::D);
                num -= 500;
            }else if num > 400{
                result.0.push(RomanDigit::C);
                result.0.push(RomanDigit::D);
                num -= 400;
            }else if num > 100{
                result.0.push(RomanDigit::C);
                num -= 100;
            }else if num > 90{
                result.0.push(RomanDigit::X);
                result.0.push(RomanDigit::C);
                num -= 90;
            }else if num > 50{
                result.0.push(RomanDigit::L);
                num -= 90;
            }else if num > 40{
                result.0.push(RomanDigit::X);
                result.0.push(RomanDigit::L);
                num -= 40;
            }else if num > 10{
                result.0.push(RomanDigit::X);
                num -= 10;
            }else if num > 9{
                result.0.push(RomanDigit::I);
                result.0.push(RomanDigit::X);
                num -= 9;
            }else if num > 5{
                result.0.push(RomanDigit::V);
                num -= 5;
            }else if num > 4{
                result.0.push(RomanDigit::I);
                result.0.push(RomanDigit::V);
                num -= 4;
            }else if num > 1{
                result.0.push(RomanDigit::I);
                num -= 1;
            }
        }

        return result;
    }
}

impl From<RomanNumber> for u32{
    fn from(num: RomanNumber) -> Self {
        let result = 0;
        for digit in num.0{
            
        }

        return result;
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
