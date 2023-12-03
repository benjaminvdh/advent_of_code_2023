use std::str::FromStr;

use aoc::Solver;

#[derive(Debug, PartialEq)]
struct PartNumber {
    x_start: usize,
    x_end: usize,
    y: usize,
    number: u32,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    x: usize,
    y: usize,
}

impl TryFrom<(usize, usize, char)> for Symbol {
    type Error = ();

    fn try_from(tuple: (usize, usize, char)) -> Result<Self, Self::Error> {
        let (x, y, c) = tuple;

        match c {
            c if !c.is_ascii_digit() && c != '.' => Ok(Self { x, y }),
            _ => Err(()),
        }
    }
}

struct Solver3;

impl Solver for Solver3 {
    type Input = (Vec<PartNumber>, Vec<Symbol>);
    type Output1 = u32;
    type Output2 = u32;

    fn parse(input: String) -> Self::Input {
        let part_numbers = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| parse_part_number_line(y, line))
            .collect();

        let symbols = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| parse_symbol_line(y, line))
            .collect();

        (part_numbers, symbols)
    }

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        todo!()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn parse_part_number_line(y: usize, line: &str) -> Vec<PartNumber> {
    let mut part_numbers = vec![];
    let mut start_index = 0;

    loop {
        let values: Vec<(usize, char)> = line
            .char_indices()
            .skip(start_index)
            .skip_while(|(_, c)| !c.is_ascii_digit())
            .take_while(|(_, c)| c.is_ascii_digit())
            .collect();

        match (values.first(), values.last()) {
            (Some((x_start, _)), Some((x_end, _))) => {
                let chars: String = values.iter().map(|(_, c)| c).collect();
                let number = u32::from_str(&chars).unwrap();

                part_numbers.push(PartNumber {
                    x_start: *x_start,
                    x_end: *x_end,
                    y,
                    number,
                });

                start_index = x_end + 1;
            }
            _ => break,
        }
    }

    part_numbers
}

fn parse_symbol_line(y: usize, line: &str) -> Vec<Symbol> {
    line.char_indices()
        .flat_map(|(x, c)| Symbol::try_from((x, y, c)))
        .collect()
}

fn main() {
    aoc::run::<Solver3>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> (Vec<PartNumber>, Vec<Symbol>) {
        let part_numbers = vec![
            PartNumber {
                x_start: 0,
                x_end: 2,
                y: 0,
                number: 467,
            },
            PartNumber {
                x_start: 5,
                x_end: 7,
                y: 0,
                number: 114,
            },
            PartNumber {
                x_start: 2,
                x_end: 3,
                y: 2,
                number: 35,
            },
            PartNumber {
                x_start: 6,
                x_end: 8,
                y: 2,
                number: 633,
            },
            PartNumber {
                x_start: 0,
                x_end: 2,
                y: 4,
                number: 617,
            },
            PartNumber {
                x_start: 7,
                x_end: 8,
                y: 5,
                number: 58,
            },
            PartNumber {
                x_start: 2,
                x_end: 4,
                y: 6,
                number: 592,
            },
            PartNumber {
                x_start: 6,
                x_end: 8,
                y: 7,
                number: 755,
            },
            PartNumber {
                x_start: 1,
                x_end: 3,
                y: 9,
                number: 664,
            },
            PartNumber {
                x_start: 5,
                x_end: 7,
                y: 9,
                number: 598,
            },
        ];

        let symbols = vec![
            Symbol { x: 3, y: 1 },
            Symbol { x: 6, y: 3 },
            Symbol { x: 3, y: 4 },
            Symbol { x: 5, y: 5 },
            Symbol { x: 3, y: 8 },
            Symbol { x: 5, y: 8 },
        ];

        (part_numbers, symbols)
    }

    #[test]
    fn parsing() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(Solver3::parse(input.to_string()), get_input());
    }
}
