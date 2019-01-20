use clap::App;
use ini;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn next_version(current_version: &str) -> &str {
    "0.1.1"
}

fn bump(config: ini::Ini) {
    // get the bumpversion section
    let section = config.section(Some("bumpversion".to_owned())).unwrap();

    // get config
    let current_version = section.get("current_version").unwrap();
    let files = HashSet::<&str>::from_iter(
        section
            .get("files")
            .unwrap()
            .split(",")
            .map(|item| item.trim()),
    );

    // update files
    for file in &files {
        fs::write(
            file,
            fs::read_to_string(file)
                .unwrap()
                .lines()
                .map(|line| {
                    if line.contains(current_version) {
                        line.replace(current_version, next_version(current_version))
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .expect("write file error!");
    }
}

fn main() {
    App::new("bumpversion-rs")
        .version("1.0")
        .about("bumpversion rust version!")
        .author("Xiao Tan")
        .get_matches();
}

#[cfg(test)]
mod test {
    use super::bump;
    use super::ini;

    #[test]
    fn parse() {
        let config_file = ".setup.cfg";
        let config = ini::Ini::load_from_file(config_file).unwrap();
        bump(config);
    }
}
