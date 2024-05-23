mod collect;
pub mod config;
mod data;
mod fs;
mod serve;
pub mod util;

use crate::util::Result;

pub fn run(config: config::Config) -> Result<()> {
    let verbose = &config.verbose();

    match &config.command {
        None => {}
        Some(command) => match command {
            config::Command::Collect => collect::collect(&config.root()?, verbose)?,
            config::Command::Serve { ip, port } => serve::serve(ip, *port, verbose)?,
        },
    }

    Ok(())
}
