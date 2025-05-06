#![feature(let_chains, iter_intersperse)]

use crate::action::action;
use crate::find_files::find_files;
use std::env; // read program flags
use std::path::Path;

mod action;
mod find_files;
mod rename;

pub struct Names {
    source: String,
    import_name: String,
    re_name: String,
}

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let auto: bool;
    let idx: usize;

    (auto, idx) = if args.len() == 4 && args[1] == "a" {
        (true, 2)
    } else if args.len() == 3 {
        (false, 1)
    } else {
        panic!("Missing flags <Import path> <Library path>");
    };

    let files = find_files(Path::new(&args[idx])).unwrap_or_else(|e| panic!("{e}"));

    action(
        files,
        Path::new(&args[idx]),
        Path::new(&args[idx + 1]),
        auto,
    );

    action::clean_dir(Path::new(&args[idx])).unwrap();
}
