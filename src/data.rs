use crate::config::Logger;
use crate::fail;
use crate::util::Result;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::{BTreeMap, HashMap};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// Sha256 is a bit faster than Sha1, apparently
type Sha = sha2::Sha256;
type Bytes = [u8; 32];

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
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

pub fn fmt(hash: &Hash, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[Hash](bytes:{})", hex::encode(&hash.bytes))
}
impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt(self, f)
    }
}
impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    paths: Vec<std::path::PathBuf>,
    content_size: usize,
}

// HashMap seems 10% faster: not too much
// type MyMap = HashMap<Hash, Info>;
type MyMap = BTreeMap<Hash, Info>;

#[derive(Debug)]
pub struct FileInfo {
    root: Option<PathBuf>,
    // hash2info: HashMap<Hash, Info>,
    hash2info: MyMap,
    buffer: Vec<u8>,
    sha: Sha,
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            root: None,
            hash2info: MyMap::new(),
            buffer: vec![0; 128 * 1024],
            sha: Sha::new(),
        }
    }

    pub fn add(
        &mut self,
        root: impl AsRef<Path>,
        rel: impl AsRef<Path>,
        logger: &Logger,
    ) -> Result<()> {
        logger.log(2, || {
            println!(
                "root: {}, rel: {}",
                root.as_ref().display(),
                rel.as_ref().display()
            )
        });

        match &self.root {
            None => self.root = Some(root.as_ref().to_owned()),
            Some(my_root) => {
                if root.as_ref() != my_root.as_path() {
                    fail!("Root should be set the same");
                }
            }
        }

        let fp = root.as_ref().join(rel.as_ref());
        let file_size = std::fs::metadata(&fp)?.len();

        let hash;
        if true {
            hash = self.compute_hash_fast(&fp)?;
        } else {
            hash = self.compute_hash_slow(&fp)?;
        }

        let info = self.hash2info.entry(hash.clone()).or_insert(Info {
            paths: vec![],
            // size: content.len(),
            content_size: file_size as usize,
        });

        info.paths.push(rel.as_ref().to_owned());

        let msg = bincode::serialize(info)?;
        let clone: Info = bincode::deserialize(&msg)?;
        logger.log(2, || {
            println!("msg: {} {}", msg.len(), hex::encode(&msg));
            println!("info: {:?}", info);
            println!("clone: {:?}", clone);
        });

        Ok(())
    }

    pub fn total_byte_count(&self) -> usize {
        let mut count = 0;
        for (_, info) in &self.hash2info {
            count += info.content_size;
        }
        count
    }

    pub fn to_naft(&self, mut sink: impl Write) -> Result<()> {
        write!(sink, "[FileInfo](total_size:{})", self.total_byte_count())?;
        if let Some(root) = &self.root {
            write!(sink, "(root:{})", root.display())?;
        }
        writeln!(sink, "{{")?;
        for (hash, info) in &self.hash2info {
            writeln!(
                sink,
                "\t[Info](hash:{})(size:{}){{",
                hex::encode(hash.bytes),
                info.content_size
            )?;
            for path in &info.paths {
                writeln!(sink, "\t\t[Path](rel:{})", path.display())?;
            }
            writeln!(sink, "\t}}")?;
        }
        writeln!(sink, "}}")?;
        Ok(())
    }

    fn compute_hash_slow(&mut self, fp: impl AsRef<Path>) -> Result<Hash> {
        let content = std::fs::read(&fp)?;

        self.sha.update(&content);
        let hash = Hash::new(self.sha.finalize_reset());

        Ok(hash)
    }

    fn compute_hash_fast(&mut self, fp: impl AsRef<Path>) -> Result<Hash> {
        let mut f = std::fs::File::open(fp)?;
        let mut size = f.metadata()?.len();

        while size > 0 {
            let read_count = f.read(&mut self.buffer)?;

            self.sha.update(&self.buffer[0..read_count]);

            size -= read_count as u64;
        }

        let hash = Hash::new(self.sha.finalize_reset());

        Ok(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut fi = FileInfo::new();
        fi.add(".", "Cargo.toml", &Logger::new()).unwrap();
        println!("{:?}", fi);
    }
}
