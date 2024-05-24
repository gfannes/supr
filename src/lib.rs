mod collect;
pub mod config;
mod data;
mod fs;
mod run;
mod serve;
pub mod util;

use crate::util::Result;

pub fn run(config: config::Config) -> Result<()> {
    let mut logger = config.logger();

    match &config.command {
        None => {}
        Some(command) => match command {
            config::Command::Collect { verbose } => {
                collect::collect(&config.root()?, &logger.update_level(*verbose))?
            }
            config::Command::Run { ip, port, verbose } => {
                run::run(ip, *port, &logger.update_level(*verbose))?
            }
            config::Command::Serve { ip, port, verbose } => {
                serve::serve(ip, *port, &logger.update_level(*verbose))?
            }
        },
    }

    Ok(())
}
