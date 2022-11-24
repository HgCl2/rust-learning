
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

pub fn new(name: &str, age: u8, role: Role) -> Member {
    Member { name: name.to_string(), role, age }
}

impl Member {
    pub fn new(name: String, age: u8, role: Role) -> Member {
        Member { name, role, age }
    }
}
