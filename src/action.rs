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
pub fn clean_dir(dir: &Path) -> Result<(), std::io::Error> {
    let files = find_files(dir)?;

    if files.is_empty() {
        return Ok(());
    };

    for f in files {
        if let Some(p) = f.parent()
            && p != dir
        {
            fs::remove_dir_all(p)?
        }
    }
    return Ok(());
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
    let (new_name, message) = if !unknown {
        (&name.import_name, "imported:")
    } else {
        (&name.re_name, "renamed into Import folder:")
    };

    match move_f(&name.source, new_name) {
        Ok(()) => {
            println!(
                "[{}] of [{}] {} {}",
                i + 1,
                files.len(),
                message.green(),
                new_name.green()
            );
            delete_dir(source_dir, Path::new(&name.source));
        }
        Err(e) => println!("{}", e),
    }
}

fn action_man(files: &[String], source_dir: &Path, i: usize, name: Names) {
    println!(
        "[{}] of [{}]: {}?\n  'i'mport: {}\n  'r'ename: {}",
        i + 1,
        files.len(),
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
            delete_dir(source_dir, Path::new(&name.source));
        }
        Err(e) => println!("{}", e),
    };
}

pub fn action(f: Vec<PathBuf>, import_dir: &Path, target_dir: &Path, auto: bool) {
    let json = json_to_data();

    // converting Vec<PathBuf> into Vec<String>
    // not necessary after everything is refactored
    let files: Vec<String> = f.iter().map(|f| f.to_string_lossy().into()).collect();

    for i in 0..files.len() {
        let (name, unknown) = rename(&files[i], import_dir, target_dir, &json);

        if auto {
            action_auto(&files, import_dir, i, name, unknown);
        } else {
            action_man(&files, import_dir, i, name);
        }
    }
}
