use std::{env, process};
use minigrep::Config;

fn main() {
    run(get_args());
}

fn get_args() -> Config {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(0);
    });
    config
}

fn run(args: Config) {
    match minigrep::run(args) {
        Ok(result) => {
            for line in result {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
