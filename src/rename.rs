use std::path::{Path, PathBuf};

use crate::Names;
use crate::action::Network;

/// Capitalizes the first character in s.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// source: complete path, target: target dir, json: parsed json struct
pub fn rename(source: &Path, target_path: &Path, json: &Vec<Network>) -> Option<Names> {
    let target_dir = target_path.to_string_lossy().to_string();

    let filename = source.file_stem().unwrap().to_string_lossy().to_string();
    if filename.matches(".").count() < 2 {
        return None;
    }

    let (network, studio) = match studio_f(&filename, json) {
        Some((n, s)) => (n, s),
        None => {
            return None;
        }
    };

    // "studio" "24.03.30.rest.of"
    let (_, name_wo_studio) = filename.split_once(".")?;

    // "24.07.30" "rest.of"
    let (date, name) = name_wo_studio.split_at_checked(8)?;
    let date = date.replace(".", "-");
    let year = format!("20{}", date.split_at_checked(2)?.0);

    let name = name.replace(".480p", "");

    // "2024-07-30 Rest Of"
    let mut capped: String = name
        .split(".")
        .map(capitalize)
        .intersperse(String::from(" "))
        .collect();

    capped.insert_str(0, &format!("20{}", &date));

    let mut path = PathBuf::from(&capped);
    path.set_extension(source.extension()?);

    capped = format!("{}/{}/{} {}", studio, year, studio, path.display());
    if network.is_some() {
        capped = format!("{}/{}", network.unwrap(), capped);
    }

    let source_str = source.display().to_string();
    Some(Names {
        source: source_str.clone(),
        import_name: format!("{target_dir}/{capped}"),
        re_name: source_str.clone(),
    })
}

/// Return Some(Some(Network), Studio) if found
/// Otherwise returns None
pub fn studio_f(s: &str, json: &Vec<Network>) -> Option<(Option<String>, String)> {
    for net in json {
        for stu in &net.studios {
            if format!("{s}.").starts_with(&stu.xc) {
                if net.name == "None" {
                    return Some((None, stu.proper.clone()));
                }

                return Some((Some(net.name.clone()), stu.proper.clone()));
            };
        }
    }
    None
}
