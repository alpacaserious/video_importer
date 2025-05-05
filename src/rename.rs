use std::path::Path;

use crate::Names;
use crate::action::Network;

pub enum Source {
    TGX,
    GP,
    Proper,
}

/// Capitalizes the first character in s.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn rename_gp(
    source: &str,
    import_dir: &str,
    target_dir: &str,
    network: &str,
    studio: &str,
) -> (Names, bool) {
    let (_, filename) = source
        .rsplit_once('/')
        .expect("failed to split filename from path");
    let mut s = filename.replace("_720p", "");
    s = s.replace("-s-", "'s "); // replace apostrophes

    let s_split: Vec<&str> = str::split(&s, '-').collect(); // split by '-'

    // capitalize all words
    let mut split_cap: Vec<String> = vec![];
    for s in s_split {
        split_cap.push(capitalize(s));
    }

    let (year, ext) = &split_cap[split_cap.len() - 1]
        .rsplit_once('.')
        .expect("failed to split filename from ext");
    let day = &split_cap[split_cap.len() - 2];
    let month = &split_cap[split_cap.len() - 3];

    // change to use Option<network>
    let mut combined = format!(
        "{}/{}/{}/{}/{} {}-{}-{}",
        target_dir, network, studio, year, studio, year, month, day
    );

    for s in split_cap.iter().take(split_cap.len() - 3) {
        combined = format!("{} {}", combined, s); // add the rest of the filename
    }

    combined = format!("{}.{}", combined, ext); // add "." and the file extension (last element)

    let (_, re_named_file) = combined
        .rsplit_once('/')
        .expect("failed to split filename from path"); // rename filename
    let re_named = format!("{}/{}", import_dir, re_named_file); // combining the Import directory and the renamed filename

    (
        Names {
            source: source.to_string(),
            import_name: combined.to_string(),
            re_name: re_named,
        },
        false,
    ) // return new names
}

fn rename_tgx(
    source: &str,
    import_dir: &str,
    target_dir: &str,
    network: &str,
    studio: &str,
) -> (Names, bool) {
    let (_, filename) = source
        .rsplit_once('/')
        .expect("failed to split filename from path");
    Path::new("/foo/bar.txt").file_name().unwrap();

    // Remove torrent info garbage
    let s = filename.replace(".XXX.1080p.HEVC.x265.PRT", "");

    // studio.year.month.day.firstname.lastname.title*n.ext*
    let s_split: Vec<&str> = str::split(&s, '.').collect();

    if s_split.len() < 3 {
        return (
            Names {
                source: source.to_string(),
                import_name: source.to_string(),
                re_name: source.to_string(),
            },
            false,
        ); // return new names
    }

    // change to use Option<network>
    let mut combined = format!(
        "{}/{}/{}/20{}/{} 20{}-{}-{}",
        target_dir, network, studio, s_split[1], studio, s_split[1], s_split[2], s_split[3]
    );

    for s in s_split.iter().take(s_split.len() - 1).skip(4) {
        combined = format!("{} {}", combined, s); // add the rest of the filename
    }

    // add "." and the file extension (last element)
    combined = format!("{}.{}", combined, s_split[s_split.len() - 1]);

    let (_, re_named_file) = combined
        .rsplit_once('/')
        .expect("failed to split filename from path"); // rename filename
    let re_named = format!("{}/{}", import_dir, re_named_file); // combining the Import directory and the renamed filename

    (
        Names {
            source: source.to_string(),
            import_name: combined.to_string(),
            re_name: re_named,
        },
        false,
    ) // return new names
}

/// source: complete path, target: target dir, json: parsed json struct
pub fn rename(
    source: &String,
    import_path: &Path,
    target_path: &Path,
    json: &Vec<Network>,
) -> (Names, bool) {
    let import_dir = import_path.to_string_lossy().to_string();
    let target_dir = target_path.to_string_lossy().to_string();

    // trim filename at the last "/"
    let (parent_dir, filename_ref) = source
        .rsplit_once('/')
        .expect("failed to split filename from path");

    let downloaded_from: Source;
    let network: String;
    let studio: String;

    let mut filename = String::from(filename_ref);

    match studio_f(&mut filename, json) {
        Some((d, n, s)) => {
            downloaded_from = d;
            network = n;
            studio = s;
        }
        None => {
            return (
                Names {
                    source: source.to_string(),
                    import_name: source.to_string(),
                    re_name: source.to_string(),
                },
                true,
            ); // return unchanged name
        }
    };

    let gp_name = format!("{}/{}", parent_dir, filename);

    match downloaded_from {
        Source::TGX => rename_tgx(source, &import_dir, &target_dir, &network, &studio),
        Source::GP => rename_gp(&gp_name, &import_dir, &target_dir, &network, &studio),
        Source::Proper => {
            (
                Names {
                    source: source.to_string(),
                    import_name: source.to_string(),
                    re_name: source.to_string(),
                },
                true,
            ) // return unchanged name
        }
    }
}

/// Return Some(Download source, Network, Studio) if found
/// Otherwise returns None
pub fn studio_f(s: &mut String, json: &Vec<Network>) -> Option<(Source, String, String)> {
    let network: String;
    let studio: String;
    let mut downloaded_from = Source::Proper;
    let mut found = false;

    for net in json {
        for stu in &net.studios {
            if s.contains(&stu.tgx) {
                // *s = s.replace(&stu.tgx, &String::new());
                downloaded_from = Source::TGX;
                found = true;
            } else if s.contains(&stu.gp) {
                // Remove old studio name + '-'
                let rep = format!("{}{}", &stu.gp, String::from("-"));
                *s = s.replace(&rep, "");

                downloaded_from = Source::GP;
                found = true;
            } else if s.contains(&stu.proper) {
                downloaded_from = Source::Proper;
                found = true;
            };

            if found {
                network = net.name.to_owned();
                studio = stu.proper.to_owned();
                return Some((downloaded_from, network, studio));
            };
        }
    }

    None
}
