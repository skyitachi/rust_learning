extern crate minigrep;
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{}, {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        // TODO:
        println!("error happens: {}", e);
        process::exit(1);
    }


}