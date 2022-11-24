
#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

pub fn new(name: &str, age: u8) -> Boss {
    Boss { name: name.to_string(), age }
}

impl Boss {
    pub fn new(name: String, age: u8) -> Boss {
        Boss { name, age }
    }
}
