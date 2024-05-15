use crate::util::{Error, Result};
use hex::encode;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

type Bytes = [u8; 32];

#[derive(PartialEq, Eq, Hash)]
pub struct Hash {
    bytes: Bytes,
}

impl Hash {
    fn new(bytes: impl Into<Bytes>) -> Hash {
        Hash {
            bytes: bytes.into(),
        }
    }
}

mod my {
    use super::Hash;

    pub fn fmt(hash: &Hash, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Hash](bytes:{})", hex::encode(&hash.bytes))
    }
}
impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        my::fmt(self, f)
    }
}
impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        my::fmt(self, f)
    }
}

#[derive(Debug)]
struct Info {
    path: std::path::PathBuf,
    content: String,
}

#[derive(Debug)]
pub struct FileInfo {
    hash2info: HashMap<Hash, Info>,
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            hash2info: HashMap::new(),
        }
    }
    pub fn add(&mut self, dir: impl AsRef<Path>, rel: impl AsRef<Path>) -> Result<()> {
        let fp = dir.as_ref().join(rel.as_ref());
        let content = std::fs::read(&fp)?;

        let mut sha = Sha256::new();
        sha.update(&content);
        let hash = Hash::new(sha.finalize());
        println!("hash: {}", &hash);

        if let Some(prev) = self.hash2info.insert(
            hash,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut fi = FileInfo::new();
        fi.add(".", "Cargo.toml").unwrap();
        println!("{:?}", fi);
    }
}
