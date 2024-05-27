use crate::{collect, config, log, run, serve, util};

pub struct App {
    config: config::Config,
}

impl App {
    pub fn new(config: config::Config) -> App {
        App { config }
    }

    pub fn run(&self) -> util::Result<()> {
        let logger = log::Logger::from(&self.config);

        match &self.config.command {
            None => {}
            Some(command) => match command {
                config::Command::Collect { .. } => {
                    let collect = collect::Collect::try_from(&self.config)?;
                    let file_info = collect.run(&logger)?;
                    file_info.to_naft(std::io::stdout())?;
                }
                config::Command::Run { .. } => {
                    let run = run::Run::try_from(&self.config)?;
                    run.run(&logger)?;
                }
                config::Command::Serve { .. } => {
                    let serve = serve::Serve::try_from(&self.config)?;
                    serve.run(&logger)?;
                }
            },
        }

        Ok(())
    }
}
