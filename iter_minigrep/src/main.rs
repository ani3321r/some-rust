use  std::env;
use std::process; //exit the program without panicking

use iter_minigrep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else( |err |{
        eprintln!("Problem passing args: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = iter_minigrep::run(config){
        eprintln!("Application Error: {}", e); //eprintln for printing error string
        process::exit(1);
    }
}

// to get the standard output in a seperate file we use the following command
// cargo run to poem.txt > output.txt