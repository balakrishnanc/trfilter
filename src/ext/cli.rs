use ansi_term::Colour as Color;
use std::io;

use crate::filter::{self, checker};

pub mod subcmds {
    pub const SHOW: &str = "show";
    pub const CHECK: &str = "check";
    pub const SUGGEST: &str = "suggest";
    pub const UPGRADE: &str = "upgrade";
}

pub mod args {
    pub const FILTER: &str = "filter";
}

// Show the rules read listed in the roaming filter file.
pub fn cmd_show(filter_file: &str) -> io::Result<()> {
    match filter::list_rules(filter_file) {
        Ok(rules) => {
            for (pos, rule) in rules.iter().enumerate() {
                println!("» {:>3} {}", pos + 1, rule);
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

// Check the rules read specified in the roaming filter file.
pub fn cmd_check(filter_file: &str) -> io::Result<()> {
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
            Ok(())
        }
        Err(e) => Err(e),
    }
}

// Suggest new rules, which can be added to the roaming filter file.
pub fn cmd_suggest(filter_file: &str) -> io::Result<()> {
    match filter::update_rules(filter_file) {
        Ok(rules) => {
            for rule in rules.iter() {
                println!("{}", Color::Yellow.bold().paint(format!("{}", rule)))
            }
            // Display the number of rules suggested.
            let n = rules.len();
            if n == 0 {
                eprintln!("No new rules to suggest.");
            } else if n == 1 {
                eprintln!("1 new rule suggested.");
            } else {
                eprintln!("{} new rules suggested.", n);
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

// Write or append suggested rules to the roaming filter file.
pub fn cmd_upgrade(filter_file: &str) -> io::Result<()> {
    filter::upgrade_rules(filter_file)
}
