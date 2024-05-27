use crate::{collect, fail, log, run, serve, util};

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
        #[arg(short, long)]
        verbose: Option<i32>,
    },
    Run {
        #[arg(short = 'i', long, default_value_t = String::from("localhost"))]
        ip: String,

        #[arg(short = 'p', long, default_value_t = 1234)]
        port: u32,

        #[arg(short, long)]
        verbose: Option<i32>,
    },
    Serve {
        #[arg(short = 'i', long, default_value_t = String::from("localhost"))]
        ip: String,

        #[arg(short = 'p', long, default_value_t = 1234)]
        port: u32,

        #[arg(short, long)]
        verbose: Option<i32>,
    },
}

impl Config {
    pub fn parse_from_cli() -> Self {
        clap::Parser::parse()
    }

    pub fn root(&self) -> util::Result<PathBuf> {
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

    pub fn logger(&self) -> log::Logger {
        log::Logger::new(self.verbose)
    }
}

// Conversion
impl TryFrom<&Config> for collect::Collect {
    type Error = Box<dyn std::error::Error>;
    fn try_from(config: &Config) -> util::Result<collect::Collect> {
        let collect = collect::Collect::new(config.root()?);
        Ok(collect)
    }
}

impl TryFrom<&Config> for run::Run {
    type Error = Box<dyn std::error::Error>;
    fn try_from(config: &Config) -> util::Result<run::Run> {
        if let Some(Command::Run { ip, port, .. }) = &config.command {
            let run = run::Run::new(ip, *port);
            Ok(run)
        } else {
            fail!("Expected Command::Run");
        }
    }
}

impl TryFrom<&Config> for serve::Serve {
    type Error = Box<dyn std::error::Error>;
    fn try_from(config: &Config) -> util::Result<serve::Serve> {
        if let Some(Command::Serve { ip, port, .. }) = &config.command {
            let run = serve::Serve::new(ip, *port);
            Ok(run)
        } else {
            fail!("Expected Command::Serve");
        }
    }
}

impl From<&Config> for log::Logger {
    fn from(config: &Config) -> log::Logger {
        let mut logger = log::Logger::new(config.verbose);

        match &config.command {
            None => {}
            Some(command) => {
                let level;
                match command {
                    Command::Collect { verbose } => level = verbose,
                    Command::Run { verbose, .. } => level = verbose,
                    Command::Serve { verbose, .. } => level = verbose,
                }
                logger.update_level(level);
            }
        }

        logger
    }
}
