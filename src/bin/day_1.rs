use aoc::Solver;

struct Solver1;

impl Solver for Solver1 {
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(input: String) -> Self::Input {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().sum()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn parse_line(line: &str) -> u32 {
    let mut iter = line
        .chars()
        .map(|c| u32::from(c))
        .filter(|&c| 48 <= c && c <= 57);

    let first = iter.next().unwrap();

    let mut second = first;
    while let Some(next) = iter.next() {
        second = next;
    }

    (first - 48) * 10 + second - 48
}

fn main() {
    aoc::run::<Solver1>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        r"1abc2
pqr3stu8vwx,
a1b2c3d4e5f,
treb7uchet"
            .to_string()
    }

    fn parse_input() -> Vec<u32> {
        Solver1::parse(get_input())
    }

    #[test]
    fn parsing() {
        assert_eq!(Solver1::parse(get_input()), vec![12, 38, 15, 77]);
    }

    #[test]
    fn part_1() {
        let input = parse_input();
        assert_eq!(Solver1::part_1(&input), 142);
    }
}
