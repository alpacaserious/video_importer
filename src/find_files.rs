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

pub fn find_files(path: &Path) -> Vec<PathBuf> {
    let Ok(paths) = fs::read_dir(path) else {
        return vec![];
    };

    paths
        .into_iter()
        .filter_map(Result::ok)
        .flat_map(|p| {
            if p.path().is_dir() {
                find_files(&p.path())
            } else if is_vid(&p.path()) {
                vec![p.path()]
            } else {
                vec![]
            }
        })
        .collect()
}
