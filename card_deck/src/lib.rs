use rand::Rng;
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let num: u8 = rand::thread_rng().gen_range(1u8, 5u8);
        return Suit::translate(num);
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => return Suit::Heart,
            2 => return Suit::Diamond,
            3 => return Suit::Spade,
            4 => return Suit::Club,
            _ => Suit::Heart,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let num: u8 = rand::thread_rng().gen_range(1u8, 14u8);
        return Rank::translate(num);
    }

    pub fn translate(value: u8) -> Rank{
        match value {
            1 => return Rank::Ace,
            2 => return Rank::Number(2),
            3 => return Rank::Number(3),
            4 => return Rank::Number(4),
            5 => return Rank::Number(5),
            6 => return Rank::Number(6),
            7 => return Rank::Number(7),
            8 => return Rank::Number(8),
            9 => return Rank::Number(9),
            10 => return Rank::Number(10),
            11 => return Rank::Jack,
            12 => return Rank::Queen,
            13 => return Rank::King,
            _ => Rank::Number(2),

        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
   return card.suit == Suit::Spade && card.rank == Rank::Ace;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winner() {
        let my_card = Card{
            rank: Rank::random(),
            suit: Suit::random(),
        };
        println!("{:?}", my_card);
        assert_eq!(winner_card(&my_card), false);
    }
}
