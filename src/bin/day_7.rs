use std::str::FromStr;

struct Solver;

#[derive(Debug, PartialEq)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, PartialEq)]
struct Game {
    hand: Hand,
    bid: u64,
}

impl aoc::Solver for Solver {
    type Input = Vec<Game>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        todo!()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn parse_line(line: &str) -> Game {
    let mut splits = line.split_whitespace();

    let hand = splits.next().unwrap();
    let cards: Vec<Card> = hand.chars().map(|c| Card::from(c)).collect();

    let bid = u64::from_str(splits.next().unwrap()).unwrap();

    Game {
        hand: Hand {
            cards: cards.try_into().unwrap(),
        },
        bid,
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        vec![
            Game {
                hand: Hand {
                    cards: [Card::C3, Card::C2, Card::T, Card::C3, Card::K],
                },
                bid: 765,
            },
            Game {
                hand: Hand {
                    cards: [Card::T, Card::C5, Card::C5, Card::J, Card::C5],
                },
                bid: 684,
            },
            Game {
                hand: Hand {
                    cards: [Card::K, Card::K, Card::C6, Card::C7, Card::C7],
                },
                bid: 28,
            },
            Game {
                hand: Hand {
                    cards: [Card::K, Card::T, Card::J, Card::J, Card::T],
                },
                bid: 220,
            },
            Game {
                hand: Hand {
                    cards: [Card::Q, Card::Q, Card::Q, Card::J, Card::A],
                },
                bid: 483,
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), todo!());
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
