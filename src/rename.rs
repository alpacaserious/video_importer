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
pub fn rename<'a>(
    source: &'a Path,
    target_path: &'a Path,
    json: &'a Vec<Network>,
) -> Option<Names<'a>> {
    let target_dir = target_path.to_string_lossy().to_string();

    let filename = source.file_stem().unwrap().to_string_lossy().to_string();
    if filename.matches(".").count() < 2 {
        return None;
    }

    let (network, studio) = studio_f(&filename, json)?;

    // "studio" "24.03.30.rest.of"
    let (_, name_wo_studio) = filename.split_once(".")?;

    // "24.07.30" "rest.of"
    let (date, name) = name_wo_studio.split_at_checked(8)?;
    let date = date.replace(".", "-");
    let year = format!("20{}", date.split_at_checked(2)?.0);

    let name = name.replace(".480p", "");

    // "2024-07-30 Rest Of"
    let mut capped: Vec<String> = name.split(".").map(capitalize).collect();

    if capped.len() >= 5 && capped[3] == "And" {
        capped[3] = ",".to_string()
    };

    let mut capped: String = capped.into_iter().intersperse(String::from(" ")).collect();
    // adds " , " instead of ", "
    capped = capped.replace(" , ", ", ");

    capped.insert_str(0, &format!("20{}", &date));

    let mut path = PathBuf::from(&capped);
    path.set_extension(source.extension()?);

    capped = format!("{}/{}/{} {}", studio, year, studio, path.display());
    if network.is_some() {
        capped = format!("{}/{}", network.unwrap(), capped);
    }

    let source_str = source.to_str().unwrap();
    Some(Names {
        old: source_str,
        new: format!("{target_dir}/{capped}"),
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

#[cfg(test)]
mod tests {
    use crate::action::json_to_data;

    use super::*;

    #[test]
    fn test_capitalize() {
        let s = "lowercase";
        assert_eq!(capitalize(s), String::from("Lowercase"));
    }

    #[test]
    fn test_and_to_comma() {
        let networks = json_to_data();
        let names = rename(
            &Path::new("/import/milfty.23.02.11.first.name.and.second.name.mp4"),
            &Path::new("/target"),
            &networks,
        )
        .unwrap();

        assert_eq!(
            names,
            Names {
                old: "/import/milfty.23.02.11.first.name.and.second.name.mp4",
                new: String::from(
                    "/target/Paper Street Media/MYLF/Milfty/2023/Milfty 2023-02-11 First Name, Second Name.mp4"
                )
            }
        )
    }

    #[test]
    fn test_rename_correct() {
        let networks = json_to_data();
        let names = rename(
            &Path::new("/import/milfty.23.02.11.title.mp4"),
            &Path::new("/target"),
            &networks,
        )
        .unwrap();

        assert_eq!(
            names,
            Names {
                old: "/import/milfty.23.02.11.title.mp4",
                new: String::from(
                    "/target/Paper Street Media/MYLF/Milfty/2023/Milfty 2023-02-11 Title.mp4"
                )
            }
        )
    }
}
