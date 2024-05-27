use crate::{collect, log, util};

use std::net::TcpStream;

pub struct Run {
    ip: String,
    port: u32,
    collect: collect::Collect,
}

impl Run {
    pub fn new(ip: impl Into<String>, port: u32, collect: collect::Collect) -> Run {
        Run {
            ip: ip.into(),
            port,
            collect,
        }
    }

    pub fn run(&self, logger: &log::Logger) -> util::Result<()> {
        logger.log(2, || println!("run({}, {})", &self.ip, self.port));

        let file_info = self.collect.run(logger)?;

        let address = format!("{}:{}", &self.ip, self.port);
        let stream = TcpStream::connect(address)?;

        bincode::serialize_into(stream, &file_info)?;

        Ok(())
    }
}
