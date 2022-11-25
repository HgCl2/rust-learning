

#[derive(Debug, Clone)]
pub struct Person<'a>{
	pub name: &'a str,
	pub age: u8,
}

impl Person<'_> {
	pub fn new<'a>(name: &'a str) -> Person {
        Person { name: name, age: 0 }
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
