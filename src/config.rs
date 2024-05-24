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
    verbose: i32,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Collect {
        #[arg(short, long, default_value_t = 0)]
        verbose: i32,
    },
    Run {
        #[arg(short = 'i', long, default_value_t = String::from("localhost"))]
        ip: String,

        #[arg(short = 'p', long, default_value_t = 1234)]
        port: u32,

        #[arg(short, long, default_value_t = 0)]
        verbose: i32,
    },
    Serve {
        #[arg(short = 'i', long, default_value_t = String::from("localhost"))]
        ip: String,

        #[arg(short = 'p', long, default_value_t = 1234)]
        port: u32,

        #[arg(short, long, default_value_t = 0)]
        verbose: i32,
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

    pub fn logger(&self) -> Logger {
        Logger {
            level: self.verbose,
        }
    }
}

pub struct Logger {
    level: i32,
}

impl Logger {
    pub fn new() -> Logger {
        Logger { level: 0 }
    }
    pub fn update_level(&mut self, level: i32) -> &Logger {
        self.level = std::cmp::max(self.level, level);
        self
    }
    pub fn log(&self, level: i32, cb: impl FnOnce() -> ()) {
        if self.level >= level {
            cb();
        }
    }
}
