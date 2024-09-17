use  std::env;
use std::process; //exit the program without panicking

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // the args func will give us an iterator over arguments passed to program
    // collect will turn it into a collection

    let config = Config::new(&args).unwrap_or_else( |err |{
        eprintln!("Problem passing args: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application Error: {}", e); //eprintln for printing error string
        process::exit(1);
    }
}

// to get the standard output in a seperate file we use the following command
// cargo run to poem.txt > output.txt