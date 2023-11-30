use std::env;
use std::path::PathBuf;

pub struct Args {
    pub quiet: bool,
    pub path: PathBuf,
}

pub fn parse() -> Args {
    let mut quiet = false;
    let mut path = None;

    for arg in env::args() {
        match arg.as_str() {
            "-q" | "--quiet" => quiet = true,
            p => path = Some(PathBuf::from(p)),
        }
    }

    Args {
        quiet,
        path: path.unwrap(),
    }
}
