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

/// source: complete path, target: target dir, networks: parsed json struct
pub fn rename<'a>(
    source: &'a Path,
    target_path: &'a Path,
    networks: &'a [Network],
) -> Option<Names<'a>> {
    let filename = source.file_stem().unwrap().to_str().unwrap();
    if filename.matches('.').count() < 2 {
        return None;
    }

    let (network, studio) = studio_f(filename, networks)?;

    // "studio" "24.03.30.rest.of"
    let (_, name_wo_studio) = filename.split_once('.')?;

    // "24.07.30" "rest.of"
    let (date, name) = name_wo_studio.split_at_checked(8)?;
    let date = date.replace('.', "-");
    let year = format!("20{}", date.split_at_checked(2)?.0);

    let name = name.replace(".480p", "");

    // "2024-07-30 Rest Of"
    let mut capped: Vec<String> = name.split('.').map(capitalize).collect();

    if capped.len() >= 5 && capped[3] == "And" {
        capped[3] = ",".to_string();
    }

    let mut capped: String = capped.into_iter().intersperse(String::from(" ")).collect();
    // adds " , " instead of ", "
    capped = capped.replace(" , ", ", ");

    capped.insert_str(0, &format!("20{}", &date));

    let mut path = PathBuf::from(&capped);
    path.set_extension(source.extension()?);

    capped = format!("{}/{}/{} {}", studio, year, studio, path.display());
    if let Some(net) = network {
        capped = format!("{net}/{capped}");
    }

    let target_dir = target_path.to_str().unwrap();
    Some(Names {
        old: source.to_str().unwrap(),
        new: format!("{target_dir}/{capped}"),
    })
}

/// Return Some(Some(Network), Studio) if found
/// Otherwise returns None
pub fn studio_f<'a>(s: &'a str, json: &'a [Network]) -> Option<(Option<&'a str>, &'a str)> {
    for net in json {
        for stu in &net.studios {
            if format!("{s}.").starts_with(stu.xc) {
                if net.name == "None" {
                    return Some((None, stu.proper));
                }
                return Some((Some(net.name), stu.proper));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        let s = "lowercase";
        assert_eq!(capitalize(s), String::from("Lowercase"));
    }

    #[test]
    fn test_and_to_comma() {
        let binding = embed_file::embed_string!("rename.json");
        let text = match binding {
            std::borrow::Cow::Borrowed(binding) => binding.to_owned(),
            std::borrow::Cow::Owned(binding) => binding,
        };
        let networks: Vec<Network> = serde_json::from_str(&text).unwrap();
        let names = rename(
            Path::new("/import/milfty.23.02.11.first.name.and.second.name.mp4"),
            Path::new("/target"),
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
        let binding = embed_file::embed_string!("rename.json");
        let text = match binding {
            std::borrow::Cow::Borrowed(binding) => binding.to_owned(),
            std::borrow::Cow::Owned(binding) => binding,
        };
        let networks: Vec<Network> = serde_json::from_str(&text).unwrap();
        let names = rename(
            Path::new("/import/milfty.23.02.11.title.mp4"),
            Path::new("/target"),
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
