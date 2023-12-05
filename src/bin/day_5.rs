use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Map {
    start: u64,
    dst: u64,
    len: u64,
}

impl Map {
    fn transform(&self, seed: u64) -> Option<u64> {
        if self.start <= seed && seed <= self.start + self.len {
            Some(seed - self.start + self.dst)
        } else {
            None
        }
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = (Vec<u64>, [Vec<Map>; 7]);
    type Output1 = u64;
    type Output2 = u64;

    fn parse(input: &str) -> Self::Input {
        let mut section_split = input.split("\n\n");

        let seeds = parse_seeds(section_split.next().unwrap());
        let map_1 = parse_map(section_split.next().unwrap());
        let map_2 = parse_map(section_split.next().unwrap());
        let map_3 = parse_map(section_split.next().unwrap());
        let map_4 = parse_map(section_split.next().unwrap());
        let map_5 = parse_map(section_split.next().unwrap());
        let map_6 = parse_map(section_split.next().unwrap());
        let map_7 = parse_map(section_split.next().unwrap());

        (seeds, [map_1, map_2, map_3, map_4, map_5, map_6, map_7])
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (seeds, steps) = input;

        seeds
            .iter()
            .map(|seed| {
                steps.iter().fold(*seed, |transformed, maps| {
                    maps.iter()
                        .filter_map(|map| map.transform(transformed))
                        .next()
                        .unwrap_or(transformed)
                })
            })
            .min()
            .unwrap()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn parse_seeds(section: &str) -> Vec<u64> {
    let prefix = "seeds: ";
    let section = &section[prefix.len()..];

    section
        .split_whitespace()
        .map(|number| u64::from_str(number).unwrap())
        .collect()
}

fn parse_map(section: &str) -> Vec<Map> {
    section
        .lines()
        .skip(1)
        .map(|line| parse_map_line(line))
        .collect()
}

fn parse_map_line(line: &str) -> Map {
    let mut numbers = line
        .split_whitespace()
        .map(|number| u64::from_str(number).unwrap());

    Map {
        dst: numbers.next().unwrap(),
        start: numbers.next().unwrap(),
        len: numbers.next().unwrap(),
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<(u64, u64, u64)> for Map {
        fn from(tuple: (u64, u64, u64)) -> Self {
            Self {
                dst: tuple.0,
                start: tuple.1,
                len: tuple.2,
            }
        }
    }

    fn get_input() -> <Solver as aoc::Solver>::Input {
        let seeds = vec![79, 14, 55, 13];
        let maps = [
            vec![(50, 98, 2).into(), (52, 50, 48).into()],
            vec![(0, 15, 37).into(), (37, 52, 2).into(), (39, 0, 15).into()],
            vec![
                (49, 53, 8).into(),
                (0, 11, 41).into(),
                (42, 0, 7).into(),
                (57, 7, 4).into(),
            ],
            vec![(88, 18, 7).into(), (18, 25, 70).into()],
            vec![
                (45, 77, 23).into(),
                (81, 45, 19).into(),
                (68, 64, 13).into(),
            ],
            vec![(0, 69, 1).into(), (1, 0, 69).into()],
            vec![(60, 56, 37).into(), (56, 93, 4).into()],
        ];

        (seeds, maps)
    }

    #[test]
    fn parsing() {
        let input = r"seeds: 79 14 55 13

a-to-b map:
50 98 2
52 50 48

b-to-c map:
0 15 37
37 52 2
39 0 15

c-to-d map:
49 53 8
0 11 41
42 0 7
57 7 4

d-to-e map:
88 18 7
18 25 70

e-to-f map:
45 77 23
81 45 19
68 64 13

f-to-g map:
0 69 1
1 0 69

g-to-h map:
60 56 37
56 93 4";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 35);
    }

    #[test]
    fn part_2() {
        //assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
