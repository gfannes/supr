#[macro_use]
mod util;
mod config;

use crate::util::Result;
use walkdir::WalkDir;

pub fn run() -> Result<()> {
    let args = config::Config::parse_from_cli();
    if args.do_log(2) {
        println!("{:?}", args);
    }

    for entry in WalkDir::new(args.root()?) {
        let entry = entry?;
        if args.do_log(1) {
            println!("{}", entry.path().display())
        }
    }

    Ok(())
}
