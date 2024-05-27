use crate::{log, util};

use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::{BTreeMap, HashMap};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// Sha256 is a bit faster than Sha1, apparently
type Sha = sha2::Sha256;
type Bytes = [u8; 32];

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileInfo {
    root: PathBuf,
    // hash2info: HashMap<Hash, Info>,
    hash2info: MyMap,
}

impl FileInfo {
    pub fn total_byte_count(&self) -> usize {
        let mut count = 0;
        for (_, info) in &self.hash2info {
            count += info.content_size;
        }
        count
    }

    pub fn to_naft(&self, mut sink: impl Write) -> util::Result<()> {
        write!(
            sink,
            "[FileInfo](total_size:{})(root:{})",
            self.total_byte_count(),
            &self.root.display()
        )?;
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
}

pub struct Builder {
    file_info: FileInfo,
    buffer: Vec<u8>,
    sha: Sha,
}

impl Builder {
    pub fn new(root: impl Into<PathBuf>) -> Builder {
        Builder {
            file_info: FileInfo {
                root: root.into(),
                hash2info: MyMap::new(),
            },
            buffer: vec![0; 128 * 1024],
            sha: Sha::new(),
        }
    }

    pub fn build(&mut self) -> FileInfo {
        std::mem::take(&mut self.file_info)
    }

    pub fn add(&mut self, rel: impl AsRef<Path>, logger: &log::Logger) -> util::Result<()> {
        logger.log(2, || println!("rel: {}", rel.as_ref().display()));

        let fp = self.file_info.root.join(rel.as_ref());
        let file_size = std::fs::metadata(&fp)?.len();

        let hash;
        if true {
            hash = self.compute_hash_fast(&fp)?;
        } else {
            hash = self.compute_hash_slow(&fp)?;
        }

        let info = self
            .file_info
            .hash2info
            .entry(hash.clone())
            .or_insert(Info {
                paths: vec![],
                // size: content.len(),
                content_size: file_size as usize,
            });

        info.paths.push(rel.as_ref().to_owned());

        Ok(())
    }

    fn compute_hash_slow(&mut self, fp: impl AsRef<Path>) -> util::Result<Hash> {
        let content = std::fs::read(&fp)?;

        self.sha.update(&content);
        let hash = Hash::new(self.sha.finalize_reset());

        Ok(hash)
    }

    fn compute_hash_fast(&mut self, fp: impl AsRef<Path>) -> util::Result<Hash> {
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
        let mut builder = Builder::new(".");
        builder.add("Cargo.toml", &log::Logger::new(0)).unwrap();
        let file_info = builder.build();
        println!("{:?}", file_info);
    }
}
