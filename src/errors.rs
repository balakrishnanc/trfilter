use std::fmt;

#[derive(Debug,Clone)]
pub struct FileNotFoundError;

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Filter file not found")
    }
}
