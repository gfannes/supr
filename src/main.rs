fn main() -> supr::util::Result<()> {
    let config = supr::config::Config::parse_from_cli();

    supr::run(config)?;

    Ok(())
}
