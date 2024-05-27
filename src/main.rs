use supr::{app, config, util};

fn main() -> util::Result<()> {
    let config = config::Config::parse_from_cli();

    config.logger().log(2, || println!("{:?}", config));

    let app = app::App::new(config);
    app.run()?;

    Ok(())
}
