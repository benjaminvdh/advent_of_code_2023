use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Sequence(Vec<i64>);

impl From<&str> for Sequence {
    fn from(line: &str) -> Self {
        Self(
            line.split_whitespace()
                .map(|number| i64::from_str(number).unwrap())
                .collect(),
        )
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<Sequence>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| Sequence::from(line)).collect()
    }

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        todo!()
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
        vec![
            Sequence(vec![0, 3, 6, 9, 12, 15]),
            Sequence(vec![1, 3, 6, 10, 15, 21]),
            Sequence(vec![10, 13, 16, 21, 30, 45]),
        ]
    }

    #[test]
    fn parsing() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

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
