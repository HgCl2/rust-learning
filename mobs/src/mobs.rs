#[derive(Debug, CLone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub member: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32
}

impl Mob {
    pub fn recruit(&self, name: String, age: u8) {

    }

    pub fn attack(enemy: Mob) {

    }
}

pub mod boss {
    #[derive(Debug, CLone, PartialEq)]
    pub struct Boss{
        pub name:String,
        pub age: u8,
    }
}

pub mod member {
    #[derive(Debug, CLone, PartialEq)]
    pub enum Role {
        Underboss,
        Caporegime,
        Soldier,
        Associate,
    }

    #[derive(Debug, CLone, PartialEq)]
    pub struct Member {
        pub name: String,
        role: Role,
        age: u8,
    }
}