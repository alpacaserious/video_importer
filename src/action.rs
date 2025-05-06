extern crate colored;
extern crate embed_file;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::rename::rename;
use crate::{Names, find_files::find_files};

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub name: String,
    pub studios: Vec<Studio>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Studio {
    pub tgx: String,
    pub gp: String,
    pub xc: String,
    pub proper: String,
}

/// Reads and parses a json file into a struct
pub fn json_to_data() -> Vec<Network> {
    let binding = embed_file::embed_string!("rename.json");

    let text = match binding {
        std::borrow::Cow::Borrowed(binding) => binding.to_owned(),
        std::borrow::Cow::Owned(binding) => binding,
    };

    // Parse the string of data into serde_json::Value.
    let value: Vec<Network> = serde_json::from_str(&text).unwrap();

    value
}

pub fn move_f(source: &str, dest: &str) -> Result<(), std::io::Error> {
    // create year dir if it does not exist
    if let Some(dest_path) = Path::new(dest).parent()
        && !dest_path.exists()
        && dest_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .parse::<u32>()
            .is_ok()
    {
        fs::create_dir(dest_path)?;
    }

    fs::rename(source, dest)?;

    Ok(())
}

/// Removes all subdirectories without videos
pub fn clean_dir(dir: &Path) -> Result<(), std::io::Error> {
    let files = find_files(dir)?;

    if files.is_empty() {
        return Ok(());
    };

    for f in files {
        let p = f.parent().expect("file parent");
        if p != dir {
            fs::remove_dir_all(p)?
        } else {
            println!("won't remove {}", p.display());
        }
    }
    Ok(())
}

fn action_auto(files_len: usize, i: usize, name: &Names) {
    match move_f(&name.source, &name.import_name) {
        Ok(()) => {
            println!(
                "[{}] of [{}] {} {}",
                i + 1,
                files_len,
                "imported: ".green(),
                &name.import_name.green()
            );
        }
        Err(e) => println!("{}", e),
    }
}

fn action_man(files_len: usize, i: usize, name: &Names) {
    println!(
        "[{}] of [{}]: {}?\n  'i'mport: {}\n  'r'ename: {}",
        i + 1,
        files_len,
        name.source,
        name.import_name,
        name.re_name
    );

    let mut sel = String::new();
    io::stdin()
        .read_line(&mut sel)
        .expect("Failed to read selection");
    sel.pop(); // Remove "\n"

    let new_name = match sel.as_str() {
        "i" => &name.import_name,
        "r" => &name.re_name,
        _ => return,
    };

    match move_f(&name.source, new_name) {
        Ok(()) => {
            println!("{} {}", "moved to:".green(), new_name.green());
        }
        Err(e) => println!("{}", e),
    };
}

pub fn action(files: Vec<PathBuf>, target_dir: &Path, auto: bool) {
    let json = json_to_data();

    let names: Vec<Names> = files
        .iter()
        .filter_map(|f| rename(f, target_dir, &json))
        .collect();

    if auto {
        names
            .iter()
            .enumerate()
            .for_each(|(i, n)| action_auto(names.len(), i, n));
    } else {
        names
            .iter()
            .enumerate()
            .for_each(|(i, n)| action_man(names.len(), i, n));
    }
}
