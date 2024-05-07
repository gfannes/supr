use crate::util::{Error, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FileInfo {
    hash__info: HashMap<String, String>,
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            hash__info: HashMap::new(),
        }
    }
}

#[test]
fn test_new() {
    let mut fi = FileInfo::new();
}
