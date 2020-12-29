use std::path::{Path, PathBuf};

use super::rule::{self, Action, Pathtype, Rule};

// Version-control-systems directories.
const VCS_DIRS: [&str; 3] = [".git", ".hg", ".svn"];

// Check if the target path contains a given directory.
fn scan_for_dir(wd: &Path, repo_dir: &str) -> Vec<Rule> {
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

// Check if the target path contains any version-control systems.
pub fn scan_for_vcs(wd: &Path) -> Vec<Rule> {
    VCS_DIRS.iter().flat_map(|d| scan_for_dir(wd, d)).collect()
}
