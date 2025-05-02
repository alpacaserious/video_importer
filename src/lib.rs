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

    let auto: bool;
    let idx: usize;

    (auto, idx) = if args.len() == 4 && args[1] == "a" {
        (true, 2)
    } else if args.len() == 3 {
        (false, 1)
    } else {
        panic!("Missing flags <Import path> <Library path>");
    };

    let files = match find_files(Path::new(&args[idx])) {
        Ok(f) => f,
        Err(e) => panic!("{e}"),
    };

    // converting Vec<PathBuf> into Vec<String>
    // not necessary after everything is refactored
    let vids: Vec<String> = files.iter().map(|f| f.to_string_lossy().into()).collect();

    action(
        &vids,
        Path::new(&args[idx]),
        Path::new(&args[idx + 1]),
        auto,
    );

    action::clean_dir(Path::new(&args[idx]));
}
