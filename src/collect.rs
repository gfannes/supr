use crate::config;
use crate::data;
use crate::fs;
use crate::util::Result;

use std::path::Path;

pub fn collect(root: &Path, verbose: &config::Verbose) -> Result<()> {
    let mut file_info = data::FileInfo::new();

    let tree = fs::Tree::new(root.to_path_buf());
    for path in &tree {
        if verbose.do_log(1) {
            println!("Loading {}", path.display());
        }
        file_info.add(root, path)?;
    }

    if verbose.do_log(2) {
        println!("{:?}", &file_info);
    }
    println!("Total byte count: {}", file_info.total_byte_count());

    Ok(())
}
