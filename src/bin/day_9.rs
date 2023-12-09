use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Sequence(Vec<i64>);

impl Sequence {
    pub fn extend(&self) -> Self {
        if self.0.iter().all(|val| *val == 0) {
            let mut sequence = self.0.clone();
            sequence.push(0);
            Self(sequence)
        } else {
            let child = Self(self.get_differences());
            let extended_child = child.extend();
            let mut sequence = self.0.clone();
            sequence.push(*sequence.last().unwrap() + extended_child.last());
            Self(sequence)
        }
    }

    pub fn last(&self) -> i64 {
        *self.0.last().unwrap()
    }

    fn get_differences(&self) -> Vec<i64> {
        self.0
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect()
    }
}

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

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|sequence| sequence.extend())
            .map(|sequence| sequence.last())
            .sum()
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
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 114);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
