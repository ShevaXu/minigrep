use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // let cfg = Config::new(&args);
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("error parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        println!("App error: {e}");
        process::exit(1);
    };
}

