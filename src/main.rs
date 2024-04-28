mod cli;

fn main() {
    let args = cli::Args::parse();
    if args.do_log(1) {
        println!("{:?}", args);
    }
}
