use supr::util::Result;

fn main() -> Result<()> {
    let config = supr::config::Config::parse_from_cli();

    config.logger().log(2, || println!("{:?}", config));

    supr::run(config)?;

    Ok(())
}
