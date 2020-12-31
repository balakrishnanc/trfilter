use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};

use super::rule::Rule;

pub const CUR_DIR: &str = r".";
pub const DBL_SLASH: &str = r"//";
pub const DBL_STAR_SLASH: &str = r"**/";
pub const PATH_SEP: &str = r"/";
pub const REL_PATH: &str = r"./";

// Craft a glob pattern from the rule path to scan for files and directories
// matching the rule path.
pub fn create_glob(rule_path: &Path) -> Option<Glob> {
    let mut path = PathBuf::new();

    // Fix the start of the glob expression.
    if rule_path.starts_with(DBL_SLASH) {
        // Replace the two slashes with a search pattern for all subdirectories.
        path.push(DBL_STAR_SLASH);
        path.push(&rule_path.to_str().expect("Failed to extract rule path")[2..]);
    } else if rule_path.starts_with(PATH_SEP) {
        // Fix the rule path to anchor it to the current directory.
        path.push(CUR_DIR);
        path.push(rule_path);
    } else if !rule_path.starts_with(REL_PATH) {
        // Anchor the rule path to current directory.
        path.push(REL_PATH);
        path.push(rule_path);
    } else {
        path.push(rule_path);
    }

    let fixed_path: PathBuf = path.iter().collect();
    let glob_path: &str = fixed_path.as_path().to_str()?;
    // println!("‘{}’ -> ‘{}’", rule_path.display(), glob_path);
    Some(Glob::new(glob_path).expect("Failed to construct glob"))
}

// Build a set of globs, one for each filter rule, to scan for matching files
// and directories.
pub fn build_globset(rules: &Vec<Rule>) -> GlobSet {
    let mut glob_builder = GlobSetBuilder::new();
    for rule in rules {
        if let Some(glob) = create_glob(&rule.path.as_path()) {
            glob_builder.add(glob);
        } else {
            eprintln!("Warn: Ignoring malformed rule `{:?}`", rule.path);
        }
    }

    glob_builder
        .build()
        .expect("Failed to build globs from filter rules!")
}
