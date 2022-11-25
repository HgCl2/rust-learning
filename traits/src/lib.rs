use std::fmt::{Formatter, write};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player {
	pub fn eat<T: Food>(&mut self, food: T) {
		self.strength += food.gives();
	}
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "{}\nStrength: {}, Score: {}, Money: {}\nWeapons: {:?}", 
        self.name, self.strength, self.score, self.money, self.weapons);
    }
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64{
        return self.weight_in_kg * 4.0;
    }
}

impl Food for Meat {
    fn gives(&self) -> f64{
        return (self.weight_in_kg - self.fat_content) * 4.0 + self.fat_content * 9.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
