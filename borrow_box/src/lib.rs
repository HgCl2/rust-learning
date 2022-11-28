use std::fmt::format;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        let result = GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        };

        return Box::new(result);
    }

    pub fn read_winner(&self) -> (String, u16) {
        let p1_score = self.p1.1;
        let p2_score = self.p2.1;
        if p1_score > p2_score {
            return self.p1.clone();
        }else if p1_score < p2_score {
            return self.p2.clone();
        }else {
            return ("Same score! tied".to_string(), p1_score);
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        if self.p1.1 + self.p2.1 == self.nb_games || 
        (self.p1.0 != user_name && self.p2.0 != user_name) {
            return;
        }else if self.p1.0 == user_name {
            self.p1.1 += 1;
        }else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }

    }

    pub fn delete(self) -> String {
        return format!("game deleted: id -> {}", self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
