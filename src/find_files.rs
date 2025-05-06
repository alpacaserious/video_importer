use std::fs;
use std::path::{Path, PathBuf};

fn is_vid(v: &Path) -> bool {
    if let Some(ext) = v.extension() {
        ext == "avi"
            || ext == "m4v"
            || ext == "mkv"
            || ext == "mov"
            || ext == "mp4"
            || ext == "mpg"
            || ext == "wmv"
    } else {
        false
    }
}

// s: working dir
pub fn find_files(p: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    // list of all files
    let mut files = vec![];

    // find all contents
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
