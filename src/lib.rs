mod config;

use walkdir::WalkDir;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run() -> Result<()> {
    let args = config::Config::parse_from_cli();
    if args.do_log(1) {
        println!("{:?}", args);
    }

    for entry in WalkDir::new(args.root.as_str()) {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(entry) => {
                if args.do_log(1) {
                    println!("{}", entry.path().display())
                }
            }
        }
    }

    Ok(())
}
