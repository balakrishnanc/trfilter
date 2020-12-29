pub mod checker;
pub mod rule;

use crate::ext::util;
use rule::{Action, Pathtype, Rule};
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

pub mod defaults {
    // Default `roaming filter` path (relative to current directory).
    pub const FILTER_REL_PATH: &str = ".tresorit/Filters/roaming.filter";
}

fn parse(line: &str) -> Rule {
    Rule::from(line)
}

// Returns a vector of filter-rule entries read from the file.
pub fn list_rules(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    match util::read_lines(filename) {
        Ok(lines) => Ok(lines
            .map(|line| line.expect("Failed to read line from file"))
            .collect()),
        Err(e) => Err(e),
    }
}

// Returns a vector of filter rules read from the file.
pub fn read_rules(filename: impl AsRef<Path>) -> io::Result<Vec<Rule>> {
    match util::read_lines(filename) {
        Ok(lines) => Ok(lines
            .map(|line| parse(&line.expect("Failed to read line from file")))
            .collect()),
        Err(e) => Err(e),
    }
}

// Return rules from the filter file, if it exists, or return an empty vector.
fn mk_rules(filename: impl AsRef<Path>) -> Result<Vec<Rule>, Error> {
    match read_rules(filename) {
        Ok(rules) => Ok(rules),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Ok(vec![]),
            _ => Err(err),
        },
    }
}

const GIT_DIR: &str = ".git";

// Check if the target path contains a git repository.
pub fn scan_for_git(wd: &Path) -> Vec<Rule> {
    let mut path_buf: PathBuf = PathBuf::new();
    // Construct `git` path relative to given path.PathBuf
    path_buf.push(wd);
    path_buf.push(GIT_DIR);
    let git_path: PathBuf = path_buf.iter().collect::<PathBuf>();
    let mut rules: Vec<Rule> = vec![];
    if git_path.exists() {
        rules.push(rule::mk_simple_rule(
            Action::Ignore,
            Pathtype::Dir,
            git_path.as_path(),
        ));
    }
    rules
}

// Checks for possible updates to filter rules.
pub fn update_rules(filename: impl AsRef<Path>) -> io::Result<Vec<Rule>> {
    let mut rules: Vec<Rule> = mk_rules(filename)?;
    rules.append(&mut scan_for_git(Path::new(".")));
    Ok(rules)
}
