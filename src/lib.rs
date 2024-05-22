#[macro_use]
mod util;
mod config;
mod data;
mod fs;

use crate::util::Result;

pub fn run() -> Result<()> {
    let config = config::Config::parse_from_cli();
    if config.do_log(2) {
        println!("{:?}", config);
    }

    let mut file_info = data::FileInfo::new();

    {
        let tree = fs::Tree::new(config.root()?);
        for path in &tree {
            if config.do_log(1) {
                println!("Loading {}", path.display());
            }
            file_info.add(config.root()?, path)?;
        }
    }

    if config.do_log(2) {
        println!("{:?}", &file_info);
    }
    println!("Total byte count: {}", file_info.total_byte_count());

    Ok(())
}
