use crate::{log, util};

use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct Serve {
    ip: String,
    port: u32,
}

impl Serve {
    pub fn new(ip: impl Into<String>, port: u32) -> Serve {
        Serve {
            ip: ip.into(),
            port,
        }
    }

    pub fn run(&self, logger: &log::Logger) -> util::Result<()> {
        logger.log(2, || println!("serve({}, {})", &self.ip, self.port));

        let ip_port = format!("{}:{}", &self.ip, self.port);

        let listener = TcpListener::bind(ip_port)?;

        loop {
            logger.log(1, || println!("Waiting for incoming connection..."));

            match listener.accept() {
                Ok((tcp_stream, sock_address)) => {
                    handle_connection(tcp_stream, sock_address, logger)?
                }
                Err(err) => return Err(Box::from(err)),
            }
        }
    }
}

fn handle_connection(
    mut stream: TcpStream,
    sock_address: SocketAddr,
    logger: &log::Logger,
) -> util::Result<()> {
    logger.log(1, || println!("Received connection from {}", sock_address));

    let mut buffer = [0 as u8; 1024];

    loop {
        let size = stream.read(&mut buffer)?;
        logger.log(2, || {
            println!("Received {size} bytes: {}", hex::encode(&buffer[0..size]))
        });
        if size == 0 {
            logger.log(2, || println!("Done"));
            return Ok(());
        }
    }
}
