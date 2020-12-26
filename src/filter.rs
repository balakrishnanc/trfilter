use std::fs;
use std::path::Path;
use crate::errors::*;


pub mod defaults {
    // Default `roaming filter` path (relative to current directory).
    pub const FILTER_REL_PATH: &str = ".tresorit/Filters/roaming.filter";
}


pub fn read_rules(filename: String) -> Result<String, FileNotFoundError>{
    if !Path::new(&filename).exists() {
        return Err(FileNotFoundError);
    }

    let contents = fs::read_to_string(filename)
        .expect("Failed to read filter file!");
    Ok(contents)
}