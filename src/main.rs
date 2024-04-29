fn main() {
    if let Err(e) = supr::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
