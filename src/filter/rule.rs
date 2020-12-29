use regex::Regex;
use std::path::{Path, PathBuf};

#[derive(Debug)]
// Represents the `Sync` attribute, which specifies whether to synchronize,
// ignore, or delete the items matched by the rule.
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

#[derive(Debug)]
// Represents the `Date` attribute, which speicifies the timestamp to use for a
// synced file.
pub enum Timestamp {
    Remote,
    Local,
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

#[derive(Debug)]
// Represents the `Threading` attribute, which specifies the thread categories
// for syncing.
pub enum ThreadType {
    Norm,
    High,
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

#[derive(Debug)]
// Represents the `PathType` attribute which specifies the scope of the filter
// rule (i.e., which directories or files the concerned rule applies to).
pub enum Pathtype {
    File,
    Dir,
    All,
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

#[derive(Debug)]
pub struct Rule {
    pub action: Action,
    pub ts: Timestamp,
    pub thr: ThreadType,
    pub prio: u32,
    pub path_type: Pathtype,
    pub case_sens: bool,
    pub path: PathBuf,
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
