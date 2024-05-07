#[macro_use]
mod util;
mod config;
mod fs;

use crate::util::Result;

pub fn run() -> Result<()> {
    let args = config::Config::parse_from_cli();
    if args.do_log(2) {
        println!("{:?}", args);
    }

    let tree = fs::Tree::new(args.root()?);
    for path in &tree {
        println!("{}", path.display())
    }

    Ok(())
}
