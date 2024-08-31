fn main() {
    if let Err(e) = web_server::run_server() {
        eprintln!("Error on starting server: {:?}", e);
    }
}
