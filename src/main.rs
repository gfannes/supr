mod cli;

use walkdir::WalkDir;

fn main() {
    let args = cli::Args::parse();
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
}
