use supr::util::Result;

fn main() -> Result<()> {
    let config = supr::config::Config::parse_from_cli();

    if config.do_log(2) {
        println!("{:?}", config);
    }

    supr::run(config)?;

    Ok(())
}
