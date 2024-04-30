fn main() {
    if let Err(e) = supr::run() {
        eprintln!("Failure detected\n{}", e);
        std::process::exit(1);
    }
}
