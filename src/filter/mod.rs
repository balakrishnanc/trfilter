pub mod checker;
pub mod rule;

use crate::util;
use rule::Rule;
use std::io;
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
