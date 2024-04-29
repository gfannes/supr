use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short = 'C', long)]
    pub root: String,

    pub opt: Option<std::path::PathBuf>,

    #[arg(short, long, default_value_t = 0)]
    verbose: i32,
}

impl Config {
    pub fn parse_from_cli() -> Config {
        clap::Parser::parse()
    }
    pub fn do_log(&self, level: i32) -> bool {
        self.verbose >= level
    }
}
