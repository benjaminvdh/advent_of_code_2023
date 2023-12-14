use std::fmt::{self, Display, Formatter};

pub mod grid;

use grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rock {
    Empty,
    Rounded,
    Cubic,
}

impl Default for Rock {
    fn default() -> Self {
        Rock::Empty
    }
}

impl Display for Rock {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{}",
            match *self {
                Rock::Empty => '.',
                Rock::Rounded => 'O',
                Rock::Cubic => '#',
            }
        )
    }
}

impl From<char> for Rock {
    fn from(c: char) -> Self {
        match c {
            '.' => Rock::Empty,
            'O' => Rock::Rounded,
            '#' => Rock::Cubic,
            _ => panic!(),
        }
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Grid<Rock>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|line| line.chars().map(|c| c.into()))
            .into()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut grid = input.clone();

        for x in 0..grid.width() {
            for y in 0..grid.height() {
                if matches!(*grid.get(x, y), Rock::Rounded) {
                    let mut curr_y = y;

                    loop {
                        if curr_y == 0 {
                            break;
                        }

                        if !matches!(*grid.get(x, curr_y - 1), Rock::Empty) {
                            break;
                        }

                        *grid.get_mut(x, curr_y) = Rock::Empty;
                        *grid.get_mut(x, curr_y - 1) = Rock::Rounded;
                        curr_y -= 1;
                    }
                }
            }
        }

        let mut sum = 0;

        for x in 0..grid.width() {
            for y in 0..grid.height() {
                if matches!(*grid.get(x, y), Rock::Rounded) {
                    sum += grid.height() - y;
                }
            }
        }

        sum
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        Grid::from(
            [
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Cubic,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Cubic,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
            ]
            .into_iter(),
        )
    }

    #[test]
    fn parsing() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 136);
    }

    //    #[test]
    //    fn part_2() {
    //        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    //    }
}
