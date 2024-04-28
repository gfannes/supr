use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'C', long)]
    pub root: String,

    #[arg(short, long, default_value_t = 0)]
    verbose: i32,
}

impl Args {
    pub fn parse() -> Args {
        clap::Parser::parse()
    }
    pub fn do_log(&self, level: i32) -> bool {
        self.verbose >= level
    }
}
