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

    if args.len() != 3 {
        panic!("Missing flags <Import path> <Library path>");
    }

    println!("Finding files...");
    let files = find_files(Path::new(&args[1])).unwrap_or_else(|e| panic!("{e}"));

    action(files, Path::new(&args[2]));

    println!("Cleaning import directory...");
    action::clean_dir(Path::new(&args[1])).unwrap();
}
