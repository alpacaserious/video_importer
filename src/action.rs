extern crate colored;
extern crate embed_file;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::{fs, io, path::Path};

use crate::Names;
use crate::find_files::find_empty_dirs;
use crate::rename::rename;

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub name: String,
    pub studios: Vec<Studio>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Studio {
    pub tgx: String,
    pub gp: String,
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

pub fn move_f(source: &String, dest: &String) -> Result<(), std::io::Error> {
    // create year dir if it does not exist
    let dest_path = Path::new(dest).parent().unwrap();
    if !dest_path.exists()
        && dest_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .parse::<f64>()
            .is_ok()
    {
        fs::create_dir(dest_path)?;
    }

    fs::rename(source, dest)?;

    Ok(())
}

/// Removes all subdirectories without videos
pub fn clean_dir(dir: &Path) {
    let dirs = match find_empty_dirs(dir) {
        Ok(f) => f,
        Err(e) => panic!("{e}"),
    };

    for dir in dirs {
        println!("{}", dir.display());
    }
}

/// Deletes directory and all contents
/// import_dir: Import dir, d: path to remove
fn delete_dir(import_dir: &Path, d: &Path) {
    // should never fail as long as the videos aren't directly under the root
    let parent_dir = d.parent().expect("no parent path found");

    // as remove_dir_all() will delete dir and content we don't wan't to delete the Import dir
    // used when videos are directly in the Import dir
    if parent_dir != import_dir {
        let _ = fs::remove_dir_all(parent_dir).inspect_err(|e| println!("{}", e));
    }
}

fn action_auto(files: &[String], source_dir: &Path, i: usize, name: Names, unknown: bool) {
    if !unknown {
        match move_f(&name.source, &name.import_name) {
            Ok(()) => {
                println!(
                    "[{}] of [{}] {} {}",
                    i + 1,
                    files.len(),
                    "imported:".green(),
                    &name.import_name.green()
                );
                delete_dir(source_dir, Path::new(&name.source));
            }
            Err(e) => println!("{}", e),
        };
    } else {
        match move_f(&name.source, &name.re_name) {
            Ok(()) => {
                println!(
                    "[{}] of [{}] {} {}",
                    i + 1,
                    files.len(),
                    "renamed into Import folder:".green(),
                    name.re_name.green()
                );
                delete_dir(source_dir, Path::new(&name.source));
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn action_man(files: &[String], source_dir: &Path, i: usize, name: Names) {
    let mut sel = String::new();

    println!(
        "[{}] of [{}]: {}?\n  'i'mport: {}\n  'r'ename: {}",
        i + 1,
        files.len(),
        name.source,
        name.import_name,
        name.re_name
    );

    io::stdin()
        .read_line(&mut sel)
        .expect("Failed to read selection");
    sel.pop(); // Remove "\n"

    match sel.as_str() {
        "i" => {
            match move_f(&name.source, &name.import_name) {
                Ok(()) => {
                    println!("{} {}", "moved to:".green(), &name.import_name.green());
                    delete_dir(source_dir, Path::new(&name.source));
                }
                Err(e) => println!("{}", e),
            };
        }
        "r" => {
            match move_f(&name.source, &name.re_name) {
                Ok(()) => {
                    println!(
                        "{} {}",
                        "renamed into Import folder:".green(),
                        name.re_name.green()
                    );
                    delete_dir(source_dir, Path::new(&name.source));
                }
                Err(e) => println!("{}", e),
            };
        }
        _ => println!("{}", "did nothing".red()),
    };
}

pub fn action(files: &[String], import_dir: &Path, target_dir: &Path, auto: bool) {
    let json = json_to_data();

    for i in 0..files.len() {
        let (name, unknown) = rename(&files[i], import_dir, target_dir, &json);

        if auto {
            action_auto(files, import_dir, i, name, unknown);
        } else {
            action_man(files, import_dir, i, name);
        }
    }
}
