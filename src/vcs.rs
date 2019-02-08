use subprocess::{Popen, PopenConfig, PopenError, Redirection};

pub fn commit(files: &Vec<&str>, message: String) {
    let mut commands = vec!["git", "add"];
    for file in files.into_iter() {
        commands.push(file);
    }
    command(&commands).unwrap();
    commands = vec!["git", "commit", "-m", message.as_str()];
    command(&commands).unwrap();
}

pub fn command(commands: &Vec<&str>) -> Result<Option<String>, PopenError> {
    let mut p = Popen::create(
        commands.as_slice(),
        PopenConfig {
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    let (out, err) = p.communicate(None)?;
    let exit_status = p.wait().unwrap();
    if exit_status.success() {
        Ok(out)
    } else {
        panic!(err)
    }
}

#[cfg(test)]
mod test {
    use super::{command, commit};
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    #[test]
    fn test_commit() {
        let mut files = vec![".setup.cfg"];
        // change these file
        for file in &files {
            let mut change_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file)
                .unwrap();
            if let Err(e) = writeln!(change_file, "\n") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        commit(&files, "fuck".to_string());
        let out = command(&vec!["git", "--no-pager", "log", "-n1"])
            .unwrap()
            .unwrap();
        println!("{}", out);
        assert!(out.contains("fuck"));
        // reset
        command(&vec!["git", "reset", "HEAD^1"]).unwrap().unwrap();
        let mut commands = vec!["git", "checkout"];
        commands.append(&mut files);
        command(&commands).unwrap().unwrap();
    }
}
