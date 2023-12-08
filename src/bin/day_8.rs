use std::collections::HashMap;

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

impl Node {
    pub fn to_map_item(self) -> (String, Dest) {
        let Node { name, left, right } = self;
        (name, Dest { left, right })
    }
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

#[derive(Debug, PartialEq)]
struct Dest {
    left: String,
    right: String,
}

impl aoc::Solver for Solver {
    type Input = (Vec<Dir>, HashMap<String, Dest>);
    type Output1 = usize;
    type Output2 = u32;

    fn parse(input: &str) -> Self::Input {
        let (dirs, nodes) = input.split_once("\n\n").unwrap();

        let dirs = dirs.chars().map(|c| c.into()).collect();
        let nodes = nodes
            .lines()
            .map(|line| Node::from(line).to_map_item())
            .collect();

        (dirs, nodes)
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (dirs, nodes) = input;

        let mut num_steps = 0;
        let mut dir_iter = dirs.iter().cycle();

        let mut cur_node_name = "AAA";

        loop {
            let cur_node = nodes.get(cur_node_name).unwrap();

            match dir_iter.next().unwrap() {
                Dir::Left => cur_node_name = &cur_node.left,
                Dir::Right => cur_node_name = &cur_node.right,
            }

            num_steps += 1;

            if cur_node_name == "ZZZ" {
                break;
            }
        }

        num_steps
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
            .map(|node| node.to_map_item())
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
