use std::str::FromStr;

struct Solver;

#[derive(Debug, PartialEq)]
struct Race {
    time: u32,
    record: u32,
}

impl From<(u32, u32)> for Race {
    fn from(tuple: (u32, u32)) -> Self {
        Self {
            time: tuple.0,
            record: tuple.1,
        }
    }
}

impl aoc::Solver for Solver {
    type Input = Vec<Race>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(input: &str) -> Self::Input {
        let mut line_iter = input.lines();

        let times = parse_line(line_iter.next().unwrap());
        let records = parse_line(line_iter.next().unwrap());

        times
            .into_iter()
            .zip(records.into_iter())
            .map(|(time, record)| Race::from((time, record)))
            .collect()
    }

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        todo!()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn parse_line(line: &str) -> impl Iterator<Item = u32> + '_ {
    line.split_whitespace()
        .skip(1)
        .map(|number| u32::from_str(number).unwrap())
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        vec![
            Race { time: 7, record: 9 },
            Race {
                time: 15,
                record: 40,
            },
            Race {
                time: 30,
                record: 200,
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 288);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
