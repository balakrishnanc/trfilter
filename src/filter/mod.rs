pub mod checker;
pub mod common;
pub mod globber;
pub mod rule;
mod scanner;

use rule::Rule;
use std::collections::HashSet;
use std::io::{self, Error, ErrorKind};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use crate::ext::util;

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

// Checks for possible updates to filter rules.
pub fn update_rules(filename: impl AsRef<Path>) -> io::Result<Vec<Rule>> {
    let wd: &Path = Path::new(".");
    // When updating rules, do not change the order of existing entries.
    let old_rules: Vec<Rule> = mk_rules(filename)?;
    // Maintain a set of rule paths corresponding to the filters to avoid
    // duplicating rules.
    let mut rule_paths: HashSet<PathBuf> =
        HashSet::from_iter(old_rules.iter().map(|r| PathBuf::from(r.path.to_owned())));
    let mut new_rules: Vec<Rule> = vec![];
    for rule in scanner::scan_dir(wd)? {
        let rule_path = rule.path.to_owned();
        // Do not add duplicates!
        if !rule_paths.contains(&rule_path) {
            // New rule!
            rule_paths.insert(rule_path);
            new_rules.push(rule);
        }
    }
    Ok(new_rules)
}
