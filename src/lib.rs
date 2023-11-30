use std::fmt::Display;
use std::fs;

mod args;

use args::Args;

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse(input: String) -> Self::Input;
    fn part_1(input: &Self::Input) -> Self::Output1;
    fn part_2(input: &Self::Input) -> Self::Output2;
}

pub fn run<S: Solver>() {
    let args = args::parse();

    let file_contents = fs::read_to_string(&args.path).unwrap();
    let input = S::parse(file_contents);

    let part_1 = S::part_1(&input);
    print_result(part_1, 1, &args);

    let part_2 = S::part_2(&input);
    print_result(part_2, 2, &args);
}

fn print_result<D: Display>(result: D, part: u8, args: &Args) {
    if args.quiet {
        print!("{result}\0");
    } else {
        println!("Result of part {part}:");
        println!("{result}");
        println!();
    }
}
