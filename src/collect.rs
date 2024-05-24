use crate::config::Logger;
use crate::data;
use crate::fs;
use crate::util::Result;

use std::path::Path;

pub fn collect(root: &Path, logger: &Logger) -> Result<()> {
    let mut file_info = data::FileInfo::new();

    let tree = fs::Tree::new(root.to_path_buf());
    for path in &tree {
        logger.log(1, || println!("Loading {}", path.display()));
        file_info.add(root, path, logger)?;
    }

    logger.log(2, || println!("{:?}", &file_info));
    logger.log(0, || {
        println!("Total byte count: {}", file_info.total_byte_count())
    });

    Ok(())
}
