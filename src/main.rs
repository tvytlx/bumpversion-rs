use clap::{App, Arg};
use ini;
use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;
mod vcs;

fn generate_next_version(config: &HashMap<String, String>, part: &String) -> String {
    let mut version = parse_version(config.get("current_version").unwrap().to_string())
        .take()
        .unwrap();
    match part.as_str() {
        "major" => version.major += 1,
        "minor" => version.minor += 1,
        "patch" => {
            if version.patch != None {
                version.patch = Some(version.patch.unwrap() + 1);
            } else {
                version.patch = Some(1);
            }
        }
        _ => panic!("wrong part"),
    };
    if version.patch != None {
        format!(
            "{}.{}.{}",
            version.major,
            version.minor,
            version.patch.unwrap()
        )
    } else {
        format!("{}.{}", version.major, version.minor)
    }
}

struct SemanticVersion {
    major: i32,
    minor: i32,
    patch: Option<i32>,
}

fn parse_version(version: String) -> Option<Box<SemanticVersion>> {
    let parts = version.split(".").collect::<Vec<&str>>();
    let major = parts[0].to_string();
    let minor = parts[1].to_string();
    let mut patch = None;
    if parts.len() == 3 {
        patch = Some(parts[2].to_string().parse::<i32>().unwrap());
    }
    Some(Box::new(SemanticVersion {
        major: major.parse::<i32>().unwrap(),
        minor: minor.parse::<i32>().unwrap(),
        patch: patch,
    }))
}

fn bump(ini_config: ini::Ini, part: &String) {
    // get the bumpversion config
    let config = ini_config.section(Some("bumpversion".to_owned())).unwrap();

    // get config
    let current_version = config.get("current_version").unwrap();
    let next_version = generate_next_version(config, &part);
    let mut files = Vec::<&str>::from_iter(
        config
            .get("files")
            .unwrap()
            .split(",")
            .map(|item| item.trim()),
    );
    files.sort();
    files.dedup();

    // update files
    for file in &files {
        fs::write(
            file,
            fs::read_to_string(file)
                .unwrap()
                .split("\n")
                .map(|line| {
                    if line.contains(current_version) {
                        line.replace(current_version, next_version.as_str())
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .expect("write file error!");
    }

    // git
    if config.get("commit").unwrap() == "True" {
        // TODO: custom commit message
        let commit_message = format!("Bumpversion {} -> {}", current_version, next_version);
        vcs::commit(&files, &commit_message);
    }
}

fn main() {
    let matches = App::new("bumpversion-rs")
        .version("0.1.0")
        .about("bump your project's version !")
        .arg(
            Arg::with_name("config_file")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("part")
                .value_name("PART")
                .help("the part of the version to increase, e.g. minor")
                .required(true)
                .index(1),
        )
        .get_matches();
    let config_file = matches.value_of("config_file").unwrap_or(".setup.cfg");
    let part = matches.value_of("part").unwrap();

    let config = ini::Ini::load_from_file(config_file).unwrap();
    bump(config, &part.to_string());
}

#[cfg(test)]
mod test {
    use super::parse_version;

    #[test]
    fn test_parse_version() {
        let mut version = parse_version("1.2.3".to_string()).unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch.unwrap(), 3);

        version = parse_version("1.2".to_string()).unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, None);
    }
}
