use regex::Regex;
use std::fmt;
use std::path::{Path, PathBuf};

use super::common::*;

// Error when converting a file path pattern read from a file (e.g., .gitignore)
// to a string for transforming it later into a filter-rule path.
pub type MalformedFilePathErr = &'static str;

#[derive(Debug, PartialEq)]
// Represents the `Sync` attribute, which specifies whether to synchronize,
// ignore, or delete the items matched by the rule.
pub enum Action {
    Sync,
    Ignore,
    Junk,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Sync => write!(f, "Sync"),
            Action::Ignore => write!(f, "Ignore"),
            Action::Junk => write!(f, "Junk"),
        }
    }
}

impl From<&str> for Action {
    fn from(s: &str) -> Action {
        match s {
            "Sync" => Action::Sync,
            "Ignore" => Action::Ignore,
            "Junk" => Action::Junk,
            _ => panic!("Malformed `Sync` option: {}", s),
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
            Action::Sync // Default value for `Sync`
        }
    }
}

#[derive(Debug, PartialEq)]
// Represents the `Date` attribute, which speicifies the timestamp to use for a
// synced file.
pub enum Timestamp {
    Remote,
    Local,
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Timestamp::Remote => write!(f, "Date=Remote"),
            Timestamp::Local => write!(f, "Date=Local"),
        }
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Timestamp {
        match s {
            "Remote" => Timestamp::Remote,
            "Local" => Timestamp::Local,
            _ => panic!("Malformed `Date` option: {}", s),
        }
    }
}

impl From<&mut Vec<&str>> for Timestamp {
    fn from(attrs: &mut Vec<&str>) -> Timestamp {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?:Date=)(Remote|Local)$").unwrap();
        }

        if let Some(i) = attrs.iter().position(|x| RE.is_match(x)) {
            let attr = attrs.remove(i);
            let cap = RE.captures(attr).unwrap();
            Timestamp::from(cap.get(1).map_or("", |v| v.as_str()))
        } else {
            Timestamp::Remote // Default value for `Date`
        }
    }
}

#[derive(Debug, PartialEq)]
// Represents the `Threading` attribute, which specifies the thread categories
// for syncing.
pub enum ThreadType {
    Norm,
    High,
}

impl fmt::Display for ThreadType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreadType::Norm => write!(f, "Threading=Normal"),
            ThreadType::High => write!(f, "Threading=Priority"),
        }
    }
}

impl From<&str> for ThreadType {
    fn from(s: &str) -> ThreadType {
        match s {
            "Normal" => ThreadType::Norm,
            "Priority" => ThreadType::High,
            _ => panic!("Malformed `Thread` option: {}", s),
        }
    }
}

impl From<&mut Vec<&str>> for ThreadType {
    fn from(attrs: &mut Vec<&str>) -> ThreadType {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?:Thread=)(Normal|Priority)$").unwrap();
        }

        if let Some(i) = attrs.iter().position(|x| RE.is_match(x)) {
            let attr = attrs.remove(i);
            let cap = RE.captures(attr).unwrap();
            ThreadType::from(cap.get(1).map_or("", |v| v.as_str()))
        } else {
            ThreadType::Norm // Default value for `Threading`
        }
    }
}

// Parses the `Priority` attribute value, which specifies the priority for the
// synchronization tasks.
fn parse_prio_from(attrs: &mut Vec<&str>) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?:Priority=)([1-5])$").unwrap();
    }

    if let Some(i) = attrs.iter().position(|x| RE.is_match(x)) {
        let attr = attrs.remove(i);
        RE.captures(attr)
            .unwrap()
            .get(1)
            .map_or("", |v| v.as_str())
            .parse::<u32>()
            .unwrap()
    } else {
        3 // Default value for `Priority`
    }
}

#[derive(Debug, PartialEq)]
// Represents the `PathType` attribute which specifies the scope of the filter
// rule (i.e., which directories or files the concerned rule applies to).
pub enum Pathtype {
    File,
    Dir,
    All,
}

impl fmt::Display for Pathtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pathtype::File => write!(f, "PathType=File"),
            Pathtype::Dir => write!(f, "PathType=Directory"),
            Pathtype::All => write!(f, ""),
        }
    }
}

impl From<&str> for Pathtype {
    fn from(s: &str) -> Pathtype {
        match s {
            "File" => Pathtype::File,
            "Directory" => Pathtype::Dir,
            "Unspecified" => Pathtype::All,
            _ => panic!("Malformed `PathType` option: {}", s),
        }
    }
}

impl From<&mut Vec<&str>> for Pathtype {
    fn from(attrs: &mut Vec<&str>) -> Pathtype {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?:PathType=)?(File|Directory|Unspecified)$").unwrap();
        }

        if let Some(i) = attrs.iter().position(|x| RE.is_match(x)) {
            let attr = attrs.remove(i);
            let cap = RE.captures(attr).unwrap();
            Pathtype::from(cap.get(1).map_or("", |v| v.as_str()))
        } else {
            Pathtype::All // Default value for `PathType`
        }
    }
}

fn parse_case_sens_from(attrs: &mut Vec<&str>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?:CaseSensitive=)?(False|True)$").unwrap();
    }

    if let Some(i) = attrs.iter().position(|x| RE.is_match(x)) {
        let attr = attrs.remove(i);
        RE.captures(attr)
            .unwrap()
            .get(1)
            .map_or("", |v| v.as_str())
            .parse::<bool>()
            .unwrap()
    } else {
        false // Default value for `CaseSensitive`
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub action: Action,
    pub ts: Timestamp,
    pub thr: ThreadType,
    pub prio: u32,
    pub path_type: Pathtype,
    pub case_sens: bool,
    pub path: PathBuf,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if (self.action == Action::Ignore)
            && (self.ts == Timestamp::Remote)
            && (self.thr == ThreadType::Norm)
            && (self.prio == 3)
        {
            if self.path_type != Pathtype::File {
                write!(f, "[{}] {}", self.action, self.path.display())
            } else {
                write!(
                    f,
                    "[{}, {}] {}",
                    self.action,
                    self.path_type,
                    self.path.display()
                )
            }
        } else {
            write!(
                f,
                "[{}, {}, {}, Priority={}, {}, CaseSensitive={}] {}",
                self.action,
                self.ts,
                self.thr,
                self.prio,
                self.path_type,
                self.case_sens,
                self.path.display()
            )
        }
    }
}

// Retrieve filter rule’s attributes and the path pattern.
fn get_attrs_and_path(rule: &str) -> (&str, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[(.+)\](.+)$").unwrap();
    }
    match RE.captures(rule) {
        Some(caps) => {
            return (
                caps.get(1).map_or("", |v| v.as_str().trim()),
                caps.get(2).map_or("", |v| v.as_str().trim()),
            )
        }
        None => panic!("Malfolrmed filter rule: {}", rule),
    }
}

impl From<&str> for Rule {
    fn from(text: &str) -> Self {
        let (attrval, path) = get_attrs_and_path(text);
        let mut attrs: Vec<&str> = attrval.split(',').map(|v| v.trim()).collect();

        // Extract the different attributes.
        let act: Action = Action::from(&mut attrs);
        let ts: Timestamp = Timestamp::from(&mut attrs);
        let thr: ThreadType = ThreadType::from(&mut attrs);
        let prio: u32 = parse_prio_from(&mut attrs);
        let path_type: Pathtype = Pathtype::from(&mut attrs);
        let case_sens: bool = parse_case_sens_from(&mut attrs);

        // `attrs` should be empty by now.
        if !attrs.is_empty() {
            panic!("Malformed attribute values: {}", attrval);
        }

        Rule {
            action: act,
            ts: ts,
            thr: thr,
            prio: prio,
            path_type: path_type,
            case_sens: case_sens,
            path: Path::new(path).to_path_buf(),
        }
    }
}

// Format glob in an `ignore` file to a filter rule path.
fn format_path(fp: &Path) -> Result<String, MalformedFilePathErr> {
    let mut rule_path = String::new();
    let fp_str = fp
        .to_str()
        .ok_or("format_path: Failed to convert `&Path` to `&str`")?;
    // Fix the start of the glob expression.
    if fp_str.starts_with(PATH_SEP) {
        // Skip the leading slash and anchor it to current directory.
        rule_path.push_str(&fp_str[1..]);
    } else if fp_str.starts_with(DBL_STAR_SLASH) {
        // Use the filter-rule specific matcher.
        rule_path.push_str(DBL_SLASH);
        rule_path.push_str(&fp_str[3..]);
    } else if fp_str.starts_with(REL_PATH) {
        // Skip the relative path part and anchor it to current directory.
        rule_path.push_str(&fp_str[2..]);
    } else if fp_str.starts_with(STAR) {
        // Append the filter-rule specific pattern matcher.
        rule_path.push_str(DBL_SLASH_STAR_DOT);
        rule_path.push_str(&fp_str[2..]);
    } else {
        rule_path.push_str(&fp_str);
    }
    Ok(rule_path)
}

// Create a `Rule` by populating unspecified fields with defaults.
pub fn mk_simple_rule(
    action: Action,
    path_type: Pathtype,
    fp: &Path,
) -> Result<Rule, MalformedFilePathErr> {
    let rule_path = format_path(fp)?;
    Ok(Rule {
        action: action,
        ts: Timestamp::Remote,
        thr: ThreadType::Norm,
        prio: 3,
        path_type: path_type,
        case_sens: false,
        path: PathBuf::from(rule_path),
    })
}
