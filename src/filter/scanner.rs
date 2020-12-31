use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use super::common::*;
use super::rule::{self, Action, Pathtype, Rule};
use crate::ext::util;

// Version-control-systems directories.
const VCS_DIRS: [&str; 3] = [".git", ".hg", ".svn"];

// Check if the target path contains an ignore file which can be used to
// generate new filter rules.
pub fn scan_ignore(ign_file: &Path) -> Option<Vec<Rule>> {
    if !(ign_file.ends_with(".gitignore") || ign_file.ends_with(".hgignore")) {
        return None;
    }
    match util::read_lines(ign_file) {
        Ok(lines) => {
            let mut rules: Vec<Rule> = vec![];
            for line in lines.map(|line| line.expect("Failed to read contents of the ignore file"))
            {
                let fp = Path::new(&line);
                let path_type: Pathtype = if fp.ends_with(PATH_SEP) {
                    Pathtype::All
                } else if fp.extension().is_none() {
                    Pathtype::All
                } else {
                    Pathtype::File
                };
                let path_buf = PathBuf::from(&line);
                rules.push(
                    rule::mk_simple_rule(Action::Ignore, path_type, path_buf.as_path())
                        .expect("Failed to form a filter rule from path glob"),
                )
            }
            Some(rules)
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => None,
            _ => {
                eprintln!("Failed to read ");
                None
            }
        },
    }
}

// Check target path for files and directories that can be ignored.
pub fn scan_dir(wd: &Path) -> io::Result<Vec<Rule>> {
    let mut rules: Vec<Rule> = vec![];
    for entry in fs::read_dir(wd)? {
        let item = entry?;
        let fp = item.path();
        if fp.is_dir() {
            let basename = item.file_name();
            if VCS_DIRS.contains(
                &basename
                    .to_str()
                    .expect("Failed to get directory or file name from the path"),
            ) {
                rules.push(
                    rule::mk_simple_rule(Action::Ignore, Pathtype::Dir, fp.as_path())
                        .expect("Failed to form a filter rule from path glob"),
                );
            }
        } else if fp.is_file() {
            if let Some(new_rules) = scan_ignore(fp.as_path()) {
                for rule in new_rules {
                    rules.push(rule);
                }
            }
        }
    }
    Ok(rules)
}
