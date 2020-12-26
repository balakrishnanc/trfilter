use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod defaults {
    // Default `roaming filter` path (relative to current directory).
    pub const FILTER_REL_PATH: &str = ".tresorit/Filters/roaming.filter";
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub enum Action {
    Sync,
    Ignore,
    Junk,
}

impl From<&str> for Action {
    fn from(s: &str) -> Action {
        match s {
            "Sync" => Action::Sync,
            "Ignore" => Action::Ignore,
            "Junk" => Action::Junk,
            _ => Action::Sync,
        }
    }
}

impl From<&mut Vec<&str>> for Action {
    fn from(attrs: &mut Vec<&str>) -> Action {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?:Sync=)?(Sync|Ignore|Junk)$").unwrap();
        }

        if let Some(i) = attrs.iter().position(|x| RE.is_match(x)) {
            let attr = attrs.remove(i);
            let cap = RE.captures(attr).unwrap();
            Action::from(cap.get(1).map_or("", |v| v.as_str()))
        } else {
            Action::Sync
        }
    }
}

#[derive(Debug)]
pub enum Timestamp {
    Remote,
    Local,
}

impl From<&str> for Timestamp {
    fn from(_text: &str) -> Timestamp {
        Timestamp::Remote
    }
}

#[derive(Debug)]
pub enum ThreadType {
    Norm,
    High,
}

#[derive(Debug)]
pub struct Rule {
    pub action: Action,
    // pub ts: Timestamp,
    // pub thr: ThreadType,
    // pub prio: u32,
    pub path: String,
}

// Retrieve filter rule’s attributes and the path pattern.
fn get_attrs_and_path(rule: &str) -> (&str, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[(.+)\](.+)$").unwrap();
    }
    match RE.captures(rule) {
        Some(caps) => {
            return (
                caps.get(1).map_or("", |v| v.as_str()),
                caps.get(2).map_or("", |v| v.as_str()),
            )
        }
        None => panic!("Malfolrmed filter rule: {}", rule),
    }
}

impl From<&str> for Rule {
    fn from(text: &str) -> Self {
        let (attrval, path) = get_attrs_and_path(text);
        let mut attrs: Vec<&str> = attrval.split(',').map(|v| v.trim()).collect();
        let act: Action = Action::from(&mut attrs);
        Rule {
            action: act,
            path: path.to_string(),
        }
    }
}

fn parse(line: &str) -> Rule {
    // let fields: Vec<&str> = line
    //     .split(['[', ']', ','].as_ref())
    //     .filter(|f| !f.is_empty())
    //     .map(|f| f.trim())
    //     .collect();

    Rule::from(line)
    // // File or directory path pattern.
    // let path_pat = fields.last().unwrap().trim();
    // Rule {
    //     path: path_pat.to_string(),
    // }
}

// Returns a vector of filter rules read from the file.
pub fn read_rules(filename: impl AsRef<Path>) -> io::Result<Vec<Rule>> {
    match read_lines(filename) {
        Ok(lines) => Ok(lines.map(|line| parse(&line.unwrap())).collect()),
        Err(e) => Err(e),
    }
}
