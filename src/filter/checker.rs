use globset::{Candidate, GlobSet};
use std::collections::HashSet;
use walkdir::WalkDir;

use super::common::*;
use super::globber::*;
use super::rule::Rule;

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
