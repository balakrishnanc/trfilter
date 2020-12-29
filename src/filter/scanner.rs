use std::path::{Path, PathBuf};

use super::rule::{self, Action, Pathtype, Rule};

const GIT_DIR: &str = ".git";

// Check if the target path contains a source-code repository.
pub fn scan_for_repo(wd: &Path, repo_dir: &str) -> Vec<Rule> {
    let mut path_buf: PathBuf = PathBuf::new();
    // Construct `git` path relative to given path.PathBuf
    path_buf.push(wd);
    path_buf.push(repo_dir);
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

// Check if the target path contains a `git` repository.
pub fn scan_for_git(wd: &Path) -> Vec<Rule> {
    scan_for_repo(wd, GIT_DIR)
}
