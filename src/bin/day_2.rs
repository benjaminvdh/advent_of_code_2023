use std::str::FromStr;

use aoc::Solver;

#[derive(Debug, PartialEq)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

struct Solver2;

impl Solver for Solver2 {
    type Input = Vec<Vec<Cubes>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: String) -> Self::Input {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .enumerate()
            .filter(|(_, game)| is_game_possible(game))
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn parse_line(line: &str) -> Vec<Cubes> {
    let game = &line[line.find(": ").unwrap() + 2..];
    let grabs = game.split("; ");
    grabs.map(|grab| parse_grab(grab)).collect()
}

fn parse_grab(grab: &str) -> Cubes {
    let mut cubes = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };

    for colors in grab.split(", ") {
        let mut splits = colors.split(' ');
        let (num, color) = (splits.next().unwrap(), splits.next().unwrap());

        match color {
            "red" => cubes.red = u32::from_str(num).unwrap(),
            "green" => cubes.green = u32::from_str(num).unwrap(),
            "blue" => cubes.blue = u32::from_str(num).unwrap(),
            _ => panic!("Invalid color"),
        }
    }

    cubes
}

fn is_game_possible(game: &[Cubes]) -> bool {
    game.iter()
        .all(|grab| grab.red <= 12 && grab.green <= 13 && grab.blue <= 14)
}

fn main() {
    aoc::run::<Solver2>();
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<(u32, u32, u32)> for Cubes {
        fn from(tuple: (u32, u32, u32)) -> Self {
            Self {
                red: tuple.0,
                green: tuple.1,
                blue: tuple.2,
            }
        }
    }

    fn get_input() -> Vec<Vec<Cubes>> {
        vec![
            vec![(4, 0, 3).into(), (1, 2, 6).into(), (0, 2, 0).into()],
            vec![(0, 2, 1).into(), (1, 3, 4).into(), (0, 1, 1).into()],
            vec![(20, 8, 6).into(), (4, 13, 5).into(), (1, 5, 0).into()],
            vec![(3, 1, 6).into(), (6, 3, 0).into(), (14, 3, 15).into()],
            vec![(6, 3, 1).into(), (1, 2, 2).into()],
        ]
    }

    #[test]
    fn parsing() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(Solver2::parse(input.to_string()), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(Solver2::part_1(&get_input()), 8);
    }
}
