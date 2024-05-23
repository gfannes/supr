use crate::fail;
use crate::util::{Error, Result};
use clap::{Parser, Subcommand};
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

    #[arg(short, long, default_value_t = 0)]
    verbose_level: i32,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Collect,
    Serve {
        #[arg(short = 'i', long, default_value_t = String::from("localhost"))]
        ip: String,

        #[arg(short = 'p', long, default_value_t = 1234)]
        port: u32,
    },
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

    pub fn verbose(&self) -> Verbose {
        Verbose {
            level: self.verbose_level,
        }
    }

    pub fn do_log(&self, level: i32) -> bool {
        self.verbose().do_log(level)
    }
}

pub struct Verbose {
    level: i32,
}

impl Verbose {
    pub fn do_log(&self, level: i32) -> bool {
        self.level >= level
    }
}
