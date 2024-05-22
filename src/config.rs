use crate::util::{Error, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short = 'o', long)]
    output: Option<String>,

    #[arg(short = 'C', long)]
    root: Option<std::path::PathBuf>,

    #[arg(short = 'u', long, default_value_t = false)]
    pub include_hidden: bool,

    #[arg(short = 'U', long, default_value_t = false)]
    pub include_ignored: bool,

    #[arg(short = 'i', long)]
    ip: Option<String>,

    #[arg(short = 'p', long)]
    port: Option<u32>,

    #[arg(short, long, default_value_t = 0)]
    verbose: i32,
}

impl Config {
    pub fn parse_from_cli() -> Self {
        clap::Parser::parse()
    }

    pub fn root(&self) -> Result<PathBuf> {
        let path_buf;
        match &self.root {
            None => path_buf = std::env::current_dir()?,
            Some(root) => path_buf = root.to_owned(),
        };

        match std::fs::metadata(&path_buf) {
            Err(e) => fail!("Could not get metadata for '{}': {}", path_buf.display(), e),
            Ok(md) => {
                if !md.is_dir() {
                    fail!("Path '{}' is not a directory", path_buf.display());
                }
            }
        }

        Ok(path_buf)
    }

    pub fn do_log(&self, level: i32) -> bool {
        self.verbose >= level
    }
}
