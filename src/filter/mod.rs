pub mod checker;
pub mod globber;
pub mod rule;
mod scanner;

use crate::ext::util;

use globset::Glob;
use rule::Rule;
use std::collections::HashSet;
use std::io::{self, Error, ErrorKind};
use std::iter::FromIterator;
use std::path::Path;

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
    let mut rules: Vec<Rule> = mk_rules(filename)?;
    // Maintain a set of globs corresponding to the filters to avoid adding
    // duplicate filter rules.
    let mut existing: HashSet<Glob> = HashSet::from_iter(
        rules
            .iter()
            .map(|r| globber::create_glob(&r.path.as_path()).expect("Failed to parse rule")),
    );
    // Scan for `git` repository and related artifacts.
    'gitrules: for rule in scanner::scan_for_vcs(wd) {
        let glob: Glob = globber::create_glob(&rule.path.as_path())
            .expect("Failed while transforming a rule into a glob");
        // Do not add duplicates!
        if existing.contains(&glob) {
            continue 'gitrules;
        }
        // New rule!
        existing.insert(glob);
        rules.push(rule);
    }
    // rules.append(&mut scan_for_git(Path::new(".")));
    Ok(rules)
}
