use std::collections::HashMap;
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
    is_start: bool,
    is_end: bool,
}

impl Node {
    pub fn get_dest(&self, nodes: &HashMap<String, Node>, dirs: &[Dir]) -> (String, String) {
        let dst = dirs.iter().fold(self.name.clone(), |name, dir| match dir {
            Dir::Left => nodes.get(&name).unwrap().left.clone(),
            Dir::Right => nodes.get(&name).unwrap().right.clone(),
        });

        (self.name.clone(), dst)
    }
}

impl From<&str> for Node {
    fn from(line: &str) -> Self {
        let mut iter = line.chars();

        let name = string_from_iter(&mut iter);

        let mut iter = iter.skip(4);
        let left = string_from_iter(&mut iter);

        let mut iter = iter.skip(2);
        let right = string_from_iter(&mut iter);

        let is_start = name.chars().last().unwrap() == 'A';
        let is_end = name.chars().last().unwrap() == 'Z';

        Self {
            name,
            left,
            right,
            is_start,
            is_end,
        }
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
    type Input = (Vec<Dir>, HashMap<String, Node>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        let (dirs, nodes) = input.split_once("\n\n").unwrap();

        let dirs = dirs.chars().map(|c| c.into()).collect();
        let nodes = nodes
            .lines()
            .map(|line| {
                let node = Node::from(line);
                (node.name.clone(), node)
            })
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

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (dirs, nodes): &(_, HashMap<String, Node>) = input;

        let map: HashMap<String, String> = nodes
            .values()
            .map(|node| node.get_dest(&nodes, &dirs))
            .collect();

        let cur_nodes: Vec<&String> = nodes
            .keys()
            .filter(|name| nodes.get(*name).unwrap().is_start)
            .collect();

        let mut ccycles = HashMap::new();

        for node in cur_nodes.iter() {
            let mut cycles = Vec::<String>::new();

            let mut current = *node;

            while !cycles.contains(current) {
                cycles.push(current.to_string());
                current = map.get(current).unwrap();
            }

            cycles.push(current.to_string());

            let _ = cycles.remove(0);
            let _ = cycles.pop();

            ccycles.insert(node.to_string(), cycles);
        }

        let mut aaa = HashMap::new();

        for last_node in cur_nodes.iter() {
            let node = last_node;
            let mut num = 0;
            let mut last_node = last_node.to_string();

            for _ in 0..ccycles.get(&last_node).unwrap().len() {
                last_node = dirs.iter().fold(last_node.to_string(), |node, dir| {
                    num += 1;
                    match dir {
                        Dir::Left => nodes.get(&node).unwrap().left.to_string(),
                        Dir::Right => nodes.get(&node).unwrap().right.to_string(),
                    }
                });
            }
            aaa.insert(node, num);
        }

        let mut qqq = HashSet::new();

        for num in aaa.values() {
            let factors = get_prime_factors(*num);
            for f in factors.keys() {
                qqq.insert(*f);
            }
        }

        let product: usize = qqq.iter().product();
        product
    }
}

fn get_prime_factors(mut num: usize) -> HashMap<usize, usize> {
    let mut result = HashMap::new();

    let mut div = 2;

    while div < num {
        if num % div == 0 {
            *result.entry(div).or_insert(0) += 1;
            num /= div;
        } else {
            div += 1;
        }
    }
    *result.entry(div).or_insert(0) += 1;

    result
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
                    is_start: true,
                    is_end: false,
                },
                Node {
                    name: "BBB".to_string(),
                    left: "AAA".to_string(),
                    right: "ZZZ".to_string(),
                    is_start: false,
                    is_end: false,
                },
                Node {
                    name: "ZZZ".to_string(),
                    left: "ZZZ".to_string(),
                    right: "ZZZ".to_string(),
                    is_start: false,
                    is_end: true,
                },
            ]
            .into_iter()
            .map(|node| (node.name.clone(), node))
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

    fn get_input_2() -> <Solver as aoc::Solver>::Input {
        (
            vec![Dir::Left, Dir::Right],
            [
                Node {
                    name: "11A".to_string(),
                    left: "11B".to_string(),
                    right: "XXX".to_string(),
                    is_start: true,
                    is_end: false,
                },
                Node {
                    name: "11B".to_string(),
                    left: "XXX".to_string(),
                    right: "11Z".to_string(),
                    is_start: false,
                    is_end: false,
                },
                Node {
                    name: "11Z".to_string(),
                    left: "11B".to_string(),
                    right: "XXX".to_string(),
                    is_start: false,
                    is_end: true,
                },
                Node {
                    name: "22A".to_string(),
                    left: "22B".to_string(),
                    right: "XXX".to_string(),
                    is_start: true,
                    is_end: false,
                },
                Node {
                    name: "22B".to_string(),
                    left: "22C".to_string(),
                    right: "22C".to_string(),
                    is_start: false,
                    is_end: false,
                },
                Node {
                    name: "22C".to_string(),
                    left: "22Z".to_string(),
                    right: "22Z".to_string(),
                    is_start: false,
                    is_end: false,
                },
                Node {
                    name: "22Z".to_string(),
                    left: "22B".to_string(),
                    right: "22B".to_string(),
                    is_start: false,
                    is_end: true,
                },
                Node {
                    name: "XXX".to_string(),
                    left: "XXX".to_string(),
                    right: "XXX".to_string(),
                    is_start: false,
                    is_end: false,
                },
            ]
            .into_iter()
            .map(|node| (node.name.clone(), node))
            .collect(),
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input_2()), 6);
    }
}
