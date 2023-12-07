use std::cmp::Ordering;
use std::collections::HashMap;

use super::Card;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hand {
    cards: [Card; 5],
}

fn get_card_map(cards: &[Card]) -> HashMap<&Card, usize> {
    let mut map = HashMap::new();

    for card in cards {
        *map.entry(card).or_insert(0) += 1;
    }

    map
}

impl Hand {
    fn get_type(&self) -> Type {
        let map = get_card_map(&self.cards);

        if map.values().any(|num| *num == 5) {
            Type::FiveOfAKind
        } else if map.values().any(|num| *num == 4) {
            Type::FourOfAKind
        } else if map.values().any(|num| *num == 3) && map.values().any(|num| *num == 2) {
            Type::FullHouse
        } else if map.values().any(|num| *num == 3) {
            Type::ThreeOfAKind
        } else if map.values().filter(|num| **num == 2).count() == 2 {
            Type::TwoPair
        } else if map.values().any(|num| *num == 2) {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }
}

impl From<[Card; 5]> for Hand {
    fn from(cards: [Card; 5]) -> Self {
        Self { cards }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
