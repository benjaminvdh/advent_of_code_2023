use std::env;
use std::fs;
use std::process::Command;

#[test]
fn day_1() {
    test(env!("CARGO_BIN_EXE_day_1"), 1);
}

#[test]
fn day_2() {
    test(env!("CARGO_BIN_EXE_day_2"), 2);
}

#[test]
fn day_3() {
    test(env!("CARGO_BIN_EXE_day_3"), 3);
}

#[test]
fn day_4() {
    test(env!("CARGO_BIN_EXE_day_4"), 4);
}

#[test]
#[ignore = "too slow"]
fn day_5() {
    test(env!("CARGO_BIN_EXE_day_5"), 5);
}

#[test]
fn day_6() {
    test(env!("CARGO_BIN_EXE_day_6"), 6);
}

fn parse_string(string: &str) -> (&str, &str) {
    let mut splits = string.split_terminator("\0");
    (splits.next().unwrap(), splits.next().unwrap())
}

fn test(exe: &str, day: u8) {
    let output = Command::new(exe)
        .arg("--quiet")
        .arg(format!("data/input/{day}"))
        .output()
        .unwrap();

    assert!(output.status.success());

    let output = String::from_utf8(output.stdout).unwrap();
    let (part_1, part_2) = parse_string(&output);

    let answers = fs::read_to_string(format!("data/output/{day}")).unwrap();
    let (answer_1, answer_2) = parse_string(&answers);

    assert_eq!(part_1, answer_1);
    assert_eq!(part_2, answer_2);
}
