use crate::config;
use crate::util::Result;

pub fn serve(ip: &str, port: u32, verbose: &config::Verbose) -> Result<()> {
    if verbose.do_log(2) {
        println!("Serve: ip {} port {}", ip, port);
    }

    Ok(())
}
