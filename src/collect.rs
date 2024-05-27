use crate::{data, fs, log, util};

use std::path::PathBuf;

pub struct Collect {
    root: PathBuf,
}

impl Collect {
    pub fn new(root: impl Into<PathBuf>) -> Collect {
        Collect { root: root.into() }
    }

    pub fn run(&self, logger: &log::Logger) -> util::Result<data::FileInfo> {
        let mut file_info = data::FileInfo::new();

        let tree = fs::Tree::new(&self.root);
        for path in &tree {
            logger.log(1, || println!("Loading {}", path.display()));
            file_info.add(&self.root, path, logger)?;
        }

        logger.log(3, || println!("{:?}", &file_info));

        Ok(file_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect() -> util::Result<()> {
        Ok(())
    }
}
