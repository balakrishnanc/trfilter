use super::rule::{self, Action, Pathtype, Rule};
use globset::{Candidate, Glob, GlobSet, GlobSetBuilder};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const CUR_DIR: &str = r".";
const DBL_SLASH: &str = r"//";
const DBL_STAR_SLASH: &str = r"**/";
const PATH_SEP: &str = r"/";
const REL_PATH: &str = r"./";

// Craft a glob pattern from the rule path to scan for files and directories
// matching the rule path.
fn create_glob(rule_path: &Path) -> Option<Glob> {
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
    }

    let fixed_path: PathBuf = path.iter().collect();
    let glob_path: &str = fixed_path.as_path().to_str()?;
    // println!("‘{}’ -> ‘{}’", rule_path.display(), glob_path);
    Some(Glob::new(glob_path).expect("Failed to construct glob"))
}

// Build a set of globs, one for each filter rule, to scan for matching files
// and directories.
fn build_globset(rules: &Vec<Rule>) -> GlobSet {
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

// Check a set of globs against files and directories in the current path, and
// return the indices of globs that match any items.
pub fn check_globs(globs: &GlobSet) -> HashSet<usize> {
    let mut glob_ids: HashSet<usize> = HashSet::new();
    // Walk the directory matching the globs against each path.
    'walk: for e in WalkDir::new(CUR_DIR).into_iter().filter_map(|e| e.ok()) {
        let fp = e.path();
        for id in globs.matches_candidate(&Candidate::new(fp)).iter() {
            if glob_ids.insert(*id) && glob_ids.len() == globs.len() {
                break 'walk;
            }
        }
    }
    glob_ids
}

// Check each rule to indicate whether they match any file or directory.
pub fn check_rules(rules: &Vec<Rule>) -> HashSet<usize> {
    check_globs(&build_globset(rules))
}

// Check if the target path contains a git repository
pub fn check_for_git(wd: &Path) -> Vec<Rule> {
    let mut path_buf: PathBuf = PathBuf::new();
    // Construct `git` path relative to given path.PathBuf
    path_buf.push(wd);
    path_buf.push(".git");
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
