use crate::config::Logger;
use crate::util::Result;
use std::io::Write;
use std::net::TcpStream;

pub fn run(ip: &str, port: u32, logger: &Logger) -> Result<()> {
    logger.log(2, || println!("run({ip}, {port})"));

    let ip_port = format!("{ip}:{port}");

    let mut stream = TcpStream::connect(ip_port)?;
    stream.write("Hello".as_bytes())?;

    Ok(())
}
