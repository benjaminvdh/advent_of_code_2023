use std::collections::HashSet;

struct Solver;

#[derive(Debug, PartialEq)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl From<&str> for Node {
    fn from(line: &str) -> Node {
        let mut iter = line.chars();

        let name = string_from_iter(&mut iter);

        let mut iter = iter.skip(4);
        let left = string_from_iter(&mut iter);

        let mut iter = iter.skip(2);
        let right = string_from_iter(&mut iter);

        Self { name, left, right }
    }
}

fn string_from_iter(iter: &mut impl Iterator<Item = char>) -> String {
    String::from_iter(
        [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ]
        .iter(),
    )
}

impl aoc::Solver for Solver {
    type Input = (Vec<Dir>, HashSet<Node>);
    type Output1 = usize;
    type Output2 = u32;

    fn parse(input: &str) -> Self::Input {
        let (dirs, nodes) = input.split_once("\n\n").unwrap();

        let dirs = dirs.chars().map(|c| c.into()).collect();
        let nodes = nodes.lines().map(|line| line.into()).collect();

        (dirs, nodes)
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
        (
            vec![Dir::Left, Dir::Left, Dir::Right],
            [
                Node {
                    name: "AAA".to_string(),
                    left: "BBB".to_string(),
                    right: "BBB".to_string(),
                },
                Node {
                    name: "BBB".to_string(),
                    left: "AAA".to_string(),
                    right: "ZZZ".to_string(),
                },
                Node {
                    name: "ZZZ".to_string(),
                    left: "ZZZ".to_string(),
                    right: "ZZZ".to_string(),
                },
            ]
            .into_iter()
            .collect(),
        )
    }

    #[test]
    fn parsing() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 6);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
