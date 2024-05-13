use crate::util::{Error, Result};
use std::collections::HashMap;
use std::path::Path;

type Hash = String;

#[derive(Debug)]
struct Info {
    path: std::path::PathBuf,
    content: String,
}

#[derive(Debug)]
pub struct FileInfo {
    hash__info: HashMap<Hash, Info>,
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            hash__info: HashMap::new(),
        }
    }
    pub fn add(&mut self, dir: impl AsRef<Path>, rel: impl AsRef<Path>) -> Result<()> {
        let fp = dir.as_ref().join(rel.as_ref());
        let content = std::fs::read(&fp)?;
        if let Some(prev) = self.hash__info.insert(
            fp.to_string_lossy().into_owned(),
            Info {
                path: rel.as_ref().to_path_buf(),
                content: std::str::from_utf8(&content)?.to_owned(),
            },
        ) {
            fail!("File info already present for {}", fp.display());
        }
        Ok(())
    }
}

#[test]
fn test_new() {
    let mut fi = FileInfo::new();
    fi.add(".", "Cargo.toml").unwrap();
    println!("{:?}", fi);
}
