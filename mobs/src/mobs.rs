pub mod boss;
pub mod member;
pub use crate::mobs::member::Member;
pub use crate::mobs::boss::Boss;


#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        let new_recruit = 
            member::new(name, member::Role::Associate, age);
        self.members.push(new_recruit);
    }

    pub fn attack(&mut self, enemy: &mut Mob) {
        let attacker_power = Mob::calculate_power(self);
        let enemy_power = Mob::calculate_power(enemy);

        if attacker_power < enemy_power || attacker_power == enemy_power{
            self.members.pop();
        } else {
            enemy.members.pop();
        }

        if enemy.members.is_empty(){
            self.wealth += enemy.wealth;
            enemy.wealth = 0;
            self.cities.append(&mut enemy.cities);
        }else{
            enemy.wealth += self.wealth;
            self.wealth = 0;
            enemy.cities.append(&mut self.cities);
        }
    }

    fn calculate_power(mob:&Mob) -> usize{
        let mut result_power: usize = 0;
        for member in &mob.members{
            match member.role{
                member::Role::Underboss => result_power += 4,
                member::Role::Caporegime => result_power += 3,
                member::Role::Soldier => result_power += 2,
                member::Role::Associate => result_power += 1,
            }
        }

        return result_power;
    }

    pub fn steal(&mut self, enemy: &mut Mob, amount: u32) {
        if enemy.wealth >= amount{
            enemy.wealth -= amount;
            self.wealth += amount
        }else{
            self.wealth += enemy.wealth;
            enemy.wealth = 0;
        }
    }

    pub fn conquer_city(&mut self, players: Vec<Mob>, city_name: String, value: u8){
        for mob in players{
            for city in mob.cities{
                if city.0 == city_name{
                    return;
                }
            }
        }

        self.cities.push((city_name, value));
    }
}