use std::fs;
use std::path::{Path, PathBuf};

fn is_vid(v: &Path) -> bool {
    let ext = v.extension().unwrap();

    ext == "avi"
        || ext == "m4v"
        || ext == "mkv"
        || ext == "mov"
        || ext == "mp4"
        || ext == "mpg"
        || ext == "wmv"
}

// s: working dir
pub fn find_files(p: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    // list of all files
    let mut files = vec![];

    // find all contents
    println!("Finding files...");
    let _ = find_files_rec(p, &mut files).inspect_err(|e| println!("{}", e));

    files.sort_unstable_by_key(|a| a.to_string_lossy().to_string().to_lowercase());

    Ok(files)
}

// s: working dir
fn find_files_rec(s: &Path, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    let paths = match fs::read_dir(s) {
        Ok(p) => p,
        Err(e) => panic!("reading path failed: {}", e),
    };

    // find everything in dir
    for path in paths {
        let tmp = path?.path();

        if tmp.is_dir() {
            let _ = find_files_rec(&tmp, files);
        } else {
            // select only videos
            if is_vid(&tmp) {
                files.push(tmp.clone());
            }
        }
    }
    Ok(())
}

pub fn find_empty_dirs(p: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut dirs = vec![];
    let _ = find_empty_dirs_rec(p, &mut dirs).inspect_err(|e| println!("{}", e));

    // TODO: remove almost dupes:
    // /foo & /foo/bar both empty, only remove /foo

    dirs.sort_unstable_by_key(|a| a.to_string_lossy().to_string().to_lowercase());

    Ok(dirs)
}

/// finds empty dirs or dirs containing only empty dirs
fn find_empty_dirs_rec(p: &Path, d: &mut Vec<PathBuf>) -> Result<usize, std::io::Error> {
    let mut non_dirs: usize = 0;
    let paths = match fs::read_dir(p) {
        Ok(p) => p,
        Err(e) => panic!("reading path failed: {}", e),
    };

    for path in paths {
        let tmp = path?.path();
        if tmp.is_dir() && 0 == find_empty_dirs_rec(&tmp, d).unwrap() {
            d.push(tmp.clone());
        } else {
            non_dirs += 1;
        }
    }
    Ok(non_dirs)
}
