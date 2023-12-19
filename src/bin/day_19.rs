use std::collections::HashMap;
use std::str::FromStr;

struct Solver;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Gt,
    Lt,
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '>' => Op::Gt,
            '<' => Op::Lt,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Prop {
    X,
    M,
    A,
    S,
}

impl From<char> for Prop {
    fn from(c: char) -> Self {
        match c {
            'x' => Prop::X,
            'm' => Prop::M,
            'a' => Prop::A,
            's' => Prop::S,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Out {
    Cont(String),
    Accept,
    Reject,
}

impl From<&str> for Out {
    fn from(string: &str) -> Self {
        if string == "A" {
            Out::Accept
        } else if string == "R" {
            Out::Reject
        } else {
            Out::Cont(string.to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Cond {
    Test {
        prop: Prop,
        op: Op,
        crit: usize,
        out: Out,
    },
    Always(Out),
}

impl From<&str> for Cond {
    fn from(input: &str) -> Cond {
        if let Some(c) = input.chars().nth(1) {
            if c == '>' || c == '<' {
                let colon = input.find(':').unwrap();
                return Cond::Test {
                    prop: Prop::from(input.chars().nth(0).unwrap()),
                    op: Op::from(input.chars().nth(1).unwrap()),
                    crit: usize::from_str(&input[2..colon]).unwrap(),
                    out: Out::from(&input[colon + 1..]),
                };
            }
        }

        Cond::Always(Out::from(input))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Workflow {
    conds: Vec<Cond>,
}

fn parse_workflow(line: &str) -> (String, Workflow) {
    let (name, workflow) = line.split_once('{').unwrap();
    let workflow = &workflow[..workflow.len() - 1];
    let conds = workflow
        .split(',')
        .map(|condition| condition.into())
        .collect();
    (name.to_string(), Workflow { conds })
}

type Workflows = Vec<(String, Workflow)>;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(line: &str) -> Part {
        let line = &line[1..line.len() - 1];
        let mut splits = line.split(',');

        let x = splits.next().unwrap();
        let m = splits.next().unwrap();
        let a = splits.next().unwrap();
        let s = splits.next().unwrap();

        Self {
            x: usize::from_str(&x[2..]).unwrap(),
            m: usize::from_str(&m[2..]).unwrap(),
            a: usize::from_str(&a[2..]).unwrap(),
            s: usize::from_str(&s[2..]).unwrap(),
        }
    }
}

impl aoc::Solver for Solver {
    type Input = (Workflows, Vec<Part>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        let (workflows, parts) = input.split_once("\n\n").unwrap();
        let workflows = workflows.lines().map(|line| parse_workflow(line)).collect();
        let parts = parts.lines().map(|line| line.into()).collect();
        (workflows, parts)
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
            [
                (
                    "px".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::A,
                                op: Op::Lt,
                                crit: 2006,
                                out: Out::Cont("qkq".to_string()),
                            },
                            Cond::Test {
                                prop: Prop::M,
                                op: Op::Gt,
                                crit: 2090,
                                out: Out::Accept,
                            },
                            Cond::Always(Out::Cont("rfg".to_string())),
                        ],
                    },
                ),
                (
                    "pv".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::A,
                                op: Op::Gt,
                                crit: 1716,
                                out: Out::Reject,
                            },
                            Cond::Always(Out::Accept),
                        ],
                    },
                ),
                (
                    "lnx".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::M,
                                op: Op::Gt,
                                crit: 1548,
                                out: Out::Accept,
                            },
                            Cond::Always(Out::Accept),
                        ],
                    },
                ),
                (
                    "rfg".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::S,
                                op: Op::Lt,
                                crit: 537,
                                out: Out::Cont("gd".to_string()),
                            },
                            Cond::Test {
                                prop: Prop::X,
                                op: Op::Gt,
                                crit: 2440,
                                out: Out::Reject,
                            },
                            Cond::Always(Out::Accept),
                        ],
                    },
                ),
                (
                    "qs".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::S,
                                op: Op::Gt,
                                crit: 3448,
                                out: Out::Accept,
                            },
                            Cond::Always(Out::Cont("lnx".to_string())),
                        ],
                    },
                ),
                (
                    "qkq".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::X,
                                op: Op::Lt,
                                crit: 1416,
                                out: Out::Accept,
                            },
                            Cond::Always(Out::Cont("crn".to_string())),
                        ],
                    },
                ),
                (
                    "crn".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::X,
                                op: Op::Gt,
                                crit: 2662,
                                out: Out::Accept,
                            },
                            Cond::Always(Out::Reject),
                        ],
                    },
                ),
                (
                    "in".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::S,
                                op: Op::Lt,
                                crit: 1351,
                                out: Out::Cont("px".to_string()),
                            },
                            Cond::Always(Out::Cont("qqz".to_string())),
                        ],
                    },
                ),
                (
                    "qqz".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::S,
                                op: Op::Gt,
                                crit: 2770,
                                out: Out::Cont("qs".to_string()),
                            },
                            Cond::Test {
                                prop: Prop::M,
                                op: Op::Lt,
                                crit: 1801,
                                out: Out::Cont("hdj".to_string()),
                            },
                            Cond::Always(Out::Reject),
                        ],
                    },
                ),
                (
                    "gd".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::A,
                                op: Op::Gt,
                                crit: 3333,
                                out: Out::Reject,
                            },
                            Cond::Always(Out::Reject),
                        ],
                    },
                ),
                (
                    "hdj".to_string(),
                    Workflow {
                        conds: vec![
                            Cond::Test {
                                prop: Prop::M,
                                op: Op::Gt,
                                crit: 838,
                                out: Out::Accept,
                            },
                            Cond::Always(Out::Cont("pv".to_string())),
                        ],
                    },
                ),
            ]
            .into_iter()
            .collect(),
            vec![
                Part {
                    x: 787,
                    m: 2655,
                    a: 1222,
                    s: 2876,
                },
                Part {
                    x: 1679,
                    m: 44,
                    a: 2067,
                    s: 496,
                },
                Part {
                    x: 2036,
                    m: 264,
                    a: 79,
                    s: 2244,
                },
                Part {
                    x: 2461,
                    m: 1339,
                    a: 466,
                    s: 291,
                },
                Part {
                    x: 2127,
                    m: 1623,
                    a: 2188,
                    s: 1013,
                },
            ],
        )
    }

    #[test]
    fn parsing() {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    #[allow(unreachable_code)]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), todo!());
    }

    #[test]
    #[allow(unreachable_code)]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
