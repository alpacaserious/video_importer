#![feature(iter_intersperse)]

use crate::action::action;
use crate::find_files::find_files;
use std::env; // read program flags
use std::path::Path;

mod action;
mod find_files;
mod rename;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Names<'a> {
    old: &'a str,
    new: String,
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let idx = if args.len() == 3 {
        1
    } else {
        panic!("Missing flags <Import path> <Library path>");
    };

    println!("Finding files...");
    let files = find_files(Path::new(&args[idx]));

    action(files, Path::new(&args[idx + 1]));

    println!("Cleaning import directory...");
    action::clean_dir(Path::new(&args[idx])).unwrap();
}
