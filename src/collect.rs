use crate::config::Logger;
use crate::data;
use crate::fs;
use crate::util::Result;

use std::io;
use std::path::Path;

pub fn collect(root: &Path, logger: &Logger) -> Result<()> {
    let mut file_info = data::FileInfo::new();

    let tree = fs::Tree::new(root.to_path_buf());
    for path in &tree {
        logger.log(1, || println!("Loading {}", path.display()));
        file_info.add(root, path, logger)?;
    }

    file_info.to_naft(io::stdout())?;

    logger.log(3, || println!("{:?}", &file_info));

    Ok(())
}
