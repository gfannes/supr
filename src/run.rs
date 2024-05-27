use crate::{log, util};

pub struct Run {
    ip: String,
    port: u32,
}

impl Run {
    pub fn new(ip: impl Into<String>, port: u32) -> Run {
        Run {
            ip: ip.into(),
            port,
        }
    }

    pub fn run(&self, logger: &log::Logger) -> util::Result<()> {
        logger.log(2, || println!("run({}, {})", &self.ip, self.port));

        Ok(())
    }
}
