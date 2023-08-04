use std::env;
use std::process;

use minigrep::Config;

fn main() {
   // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });
    //dbg!(args);
    // println!("Seaching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
       eprintln!("Application error: {e}");
        process::exit(1);
    }

    
}

