use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use super::common::*;
use super::rule::{self, Action, Pathtype, Rule};
use crate::ext::util;

// Version-control-systems directories.
const VCS_DIRS: [&str; 3] = [".git", ".hg", ".svn"];

// Parse and transform a file path glob into a filter rule pattern.
pub fn parse_path_pat(fp_glob: &Path) -> PathBuf {
    let fp_str = fp_glob.to_str().expect("Failed to parse file path glob");
    let mut rule_path = PathBuf::new();
    // Fix the start of the glob expression.
    if fp_glob.starts_with(PATH_SEP) {
        // Skip the leading slash and anchor it to current directory.
        rule_path.push(&fp_str[1..]);
    } else if !rule_path.starts_with(REL_PATH) {
        // Skip the relative path part and anchor it to current directory.
        rule_path.push(&fp_str[2..]);
    } else if !rule_path.starts_with(DBL_STAR_SLASH) {
        // Use the filter-rule specific matcher.
        rule_path.push(DBL_SLASH);
        rule_path.push(&fp_str[3..]);
    } else {
        rule_path.push(fp_glob);
    }
    rule_path.iter().collect::<PathBuf>()
}

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
                let path_buf = parse_path_pat(Path::new(&line));
                rules.push(rule::mk_simple_rule(
                    Action::Ignore,
                    path_type,
                    path_buf.as_path(),
                ))
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
                rules.push(rule::mk_simple_rule(
                    Action::Ignore,
                    Pathtype::Dir,
                    fp.as_path(),
                ));
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
