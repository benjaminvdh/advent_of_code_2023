//! This library defines a framework for solving Advent of Code puzzles.
//! Copy-and-paste the code fragment below into a new src/bin/day_<number>.rs file.
//! Change the types of Input, Output1 and Output2, and fill in the todo!() statements
//! to solve the puzzles.
//!
//! ```no_run
//! struct Solver;
//!
//! impl aoc::Solver for Solver {
//!     type Input = Vec<u32>;
//!     type Output1 = u32;
//!     type Output2 = u32;
//!
//!     fn parse(_input: &str) -> Self::Input {
//!         todo!()
//!     }
//!
//!     fn part_1(_input: &Self::Input) -> Self::Output1 {
//!         todo!()
//!     }
//!
//!     fn part_2(_input: &Self::Input) -> Self::Output2 {
//!         todo!()
//!     }
//! }
//!
//! fn main() {
//!     aoc::run::<Solver>();
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     fn get_input() -> <Solver as aoc::Solver>::Input {
//!         todo!()
//!     }
//!
//!     #[test]
//!     fn parsing() {
//!         let input = todo!();
//!
//!         assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
//!     }
//!
//!     #[test]
//!     fn part_1() {
//!         assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), todo!());
//!     }
//!
//!     #[test]
//!     fn part_2() {
//!         assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
//!     }
//! }
//! ```

use std::fmt::Display;
use std::fs;

mod args;

use args::Args;

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse(input: &str) -> Self::Input;
    fn part_1(input: &Self::Input) -> Self::Output1;
    fn part_2(input: &Self::Input) -> Self::Output2;
}

pub fn run<S: Solver>() {
    let args = args::parse();

    let file_contents = fs::read_to_string(&args.path).unwrap();
    let input = S::parse(&file_contents);

    if args.run_part_1 {
        let part_1 = S::part_1(&input);
        print_result(part_1, 1, &args);
    }

    if args.run_part_2 {
        let part_2 = S::part_2(&input);
        print_result(part_2, 2, &args);
    }
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
