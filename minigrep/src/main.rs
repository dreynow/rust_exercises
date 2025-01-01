use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let words: Vec<String> = env::args().collect();

    let conf = Config::build(&words).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });



    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
