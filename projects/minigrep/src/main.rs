use minigrep;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Query: {:?}. In search of: {:?}.",
        config.filename, config.query
    );

    match minigrep::run(config) {
        Ok(()) => println!("crushing it"),
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    };
}