extern crate colored;
extern crate embed_file;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::rename::rename;
use crate::{Names, find_files::find_files};

#[derive(Serialize, Deserialize, Debug)]
pub struct Network<'a> {
    pub name: &'a str,
    pub studios: Vec<Studio<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Studio<'a> {
    pub tgx: &'a str,
    pub gp: &'a str,
    pub xc: &'a str,
    pub proper: &'a str,
}

pub fn move_f(source: &str, dest: &str) -> Result<(), std::io::Error> {
    // create year dir if it does not exist
    if let Some(dest_path) = Path::new(dest).parent()
        && let Some(studio_dir) = dest_path.parent()
        && studio_dir.exists()
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
    let files = find_files(dir);

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

fn action_impl(files_len: usize, i: usize, name: &Names) {
    match move_f(name.old, &name.new) {
        Ok(()) => {
            println!(
                "[{}] of [{}] {} {}",
                i + 1,
                files_len,
                "imported: ".green(),
                name.new.green()
            );
        }
        Err(e) => println!("{e}"),
    }
}

pub fn action(files: Vec<PathBuf>, target_dir: &Path, networks: Vec<Network>) {
    let names: Vec<Names> = files
        .iter()
        .filter_map(|f| rename(f, target_dir, &networks))
        .collect();

    names
        .iter()
        .enumerate()
        .for_each(|(i, n)| action_impl(names.len(), i, n));
}
