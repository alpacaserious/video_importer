#![feature(iter_intersperse)]
#![warn(clippy::pedantic)]
use crate::action::{Network, action};
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

    let binding = embed_file::embed_string!("rename.json");
    let text = match binding {
        std::borrow::Cow::Borrowed(binding) => binding.to_owned(),
        std::borrow::Cow::Owned(binding) => binding,
    };
    let networks: Vec<Network> = serde_json::from_str(&text).unwrap();
    action(&files, Path::new(&args[idx + 1]), &networks);

    println!("Cleaning import directory...");
    action::clean_dir(Path::new(&args[idx])).unwrap();
}
