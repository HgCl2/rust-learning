
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

impl Member {
    pub fn new(name: String, age: u8, role: Role) -> Member {
        Member { name, role, age }
    }
}
