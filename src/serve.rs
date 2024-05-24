use crate::config::Logger;
use crate::util::Result;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn serve(ip: &str, port: u32, logger: &Logger) -> Result<()> {
    logger.log(2, || println!("serve({ip}, {port})"));

    let ip_port = format!("{ip}:{port}");

    let listener = TcpListener::bind(ip_port)?;

    loop {
        logger.log(1, || println!("Waiting for incoming connection..."));

        match listener.accept() {
            Ok((tcp_stream, sock_address)) => handle_connection(tcp_stream, sock_address, logger)?,
            Err(err) => return Err(Box::from(err)),
        }
    }
}

fn handle_connection(
    mut stream: TcpStream,
    sock_address: SocketAddr,
    logger: &Logger,
) -> Result<()> {
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
