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

    for entry in ignore::WalkBuilder::new(args.root()?)
        .hidden(!args.include_hidden)
        .ignore(args.include_ignored)
        .build()
    {
        let entry = entry?;
        if args.do_log(1) {
            println!("{}", entry.path().display())
        }
    }

    Ok(())
}
