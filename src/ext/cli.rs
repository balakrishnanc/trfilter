use ansi_term::Colour as Color;
use std::path::Path;

use crate::filter::{self, checker};

pub mod subcmds {
    pub const SHOW: &str = "show";
    pub const CHECK: &str = "check";
    pub const SUGGEST: &str = "suggest";
}

pub mod args {
    pub const FILTER: &str = "filter";
}

// Show the rules read listed in the roaming filter file.
pub fn cmd_show(filter_file: &str) {
    match filter::list_rules(filter_file) {
        Ok(rules) => {
            for (pos, rule) in rules.iter().enumerate() {
                println!("» {:>3} {}", pos + 1, rule)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Check the rules read specified in the roaming filter file.
pub fn cmd_check(filter_file: &str) {
    match filter::read_rules(filter_file) {
        Ok(rules) => {
            let matches = checker::check_rules(&rules);
            for (i, rule) in rules.iter().enumerate() {
                let msg = match matches.contains(&i) {
                    true => Color::Green
                        .bold()
                        .paint(format!("+ {}", rule.path.display())),
                    false => Color::Red.paint(format!("- {}", rule.path.display())),
                };
                println!("{:>3} {}", i + 1, msg);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Suggest new rules, which can be added to the roaming filter file.
pub fn cmd_suggest(_filter_file: &str) {
    println!("{:?}", checker::check_for_git(Path::new(".")));
    let svn_path = Path::new(".svn");
    if svn_path.exists() {
        println!("{}", svn_path.display());
    }
    let hg_path = Path::new(".hg");
    if hg_path.exists() {
        println!("{}", hg_path.display());
    }
}
