use std::process;
use eventsystem::Config;

fn main() {
    let config = Config::parse();
    //let config = Config::dummy();
    if let Err(e) = eventsystem::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}