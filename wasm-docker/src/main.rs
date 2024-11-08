fn main() {
    if let Err(e) = ferris_says::say("Wasm in Docker", 80, &mut std::io::stdout()) {
        eprintln!("Error: {e}", );
    }
}
