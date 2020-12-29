use crate::filter::{self, checker};
use ansi_term::Colour as Color;

pub mod subcmds {
    pub const SHOW: &str = "show";
    pub const CHECK: &str = "check";
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
                let p = format!("{}", rule.path.display());
                match matches.contains(&i) {
                    true => println!("{:>3} {}", i + 1, Color::Red.paint(p)),
                    false => println!("{:>3} {}", i + 1, Color::Yellow.paint(p)),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
