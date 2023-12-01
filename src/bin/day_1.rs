use aoc::Solver;

struct Solver1;

impl Solver for Solver1 {
    type Input = Vec<String>;
    type Output1 = u32;
    type Output2 = usize;

    fn parse(input: String) -> Self::Input {
        input.lines().map(|line| line.to_string()).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|line| parse_line_part_1(line)).sum()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.iter().map(|line| parse_line_part_2(line)).sum()
    }
}

fn parse_line_part_1(line: &str) -> u32 {
    let mut iter = line
        .chars()
        .map(|c| u32::from(c))
        .filter(|&c| 48 <= c && c <= 57)
        .map(|c| c - 48);

    let first = iter.next().unwrap();
    let last = iter.next_back().unwrap_or(first);

    first * 10 + last
}

fn parse_line_part_2(line: &str) -> usize {
    const NUMBER_STRINGS: [&str; 20] = [
        "zero", "0", "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6",
        "seven", "7", "eight", "8", "nine", "9",
    ];

    let mut first = None;
    let mut last = None;

    for (number_index, number_string) in NUMBER_STRINGS.iter().enumerate() {
        for (current_index, _) in line.match_indices(number_string) {
            if first.map_or(true, |(first_index, _)| current_index < first_index) {
                first = Some((current_index, number_index / 2));
            }

            if last.map_or(true, |(last_index, _)| current_index > last_index) {
                last = Some((current_index, number_index / 2));
            }
        }
    }

    first.unwrap().1 * 10 + last.unwrap().1
}

fn main() {
    aoc::run::<Solver1>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();
        assert_eq!(Solver1::part_1(&Solver1::parse(input)), 142);
    }

    #[test]
    fn part_2() {
        let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();
        assert_eq!(Solver1::part_2(&Solver1::parse(input)), 281);
    }
}
