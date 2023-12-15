struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input
            .split(',')
            .map(|string| string.trim().to_owned())
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|seq| run_hash(seq)).sum()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn run_hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        vec![
            String::from("rn=1"),
            String::from("cm-"),
            String::from("qp=3"),
            String::from("cm=2"),
            String::from("qp-"),
            String::from("pc=4"),
            String::from("ot=9"),
            String::from("ab=5"),
            String::from("pc-"),
            String::from("pc=6"),
            String::from("ot=7"),
        ]
    }

    #[test]
    fn parsing() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 1320);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
