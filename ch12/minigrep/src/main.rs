use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // main is now just for error handling and argument collecting
    let args: Vec<String> = env::args().collect();

    // create a vector, composed of strings from command line arguments, collect into the vector (need to collect because args is an iterator)

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // create a Config instance, populate it with values from the command line arguments
    // Err 'not enough arguments' would get passed here in else condition <3
    // use unwrap or else, unwrap gets the value out of the Ok result in build to make it accessible, unwraps it and gives it to Config here

    eprintln!("Searching for {}", config.query);
    eprintln!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // we don't use unwrap here, just if let config::run Error because we don't need unwrapping to get the value from Ok or Error, Ok just returns () in run function
}
