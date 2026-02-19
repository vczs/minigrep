use std::env;
use minigrep::Config;

fn main() {
     let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("build config err: {err}");
        std::process::exit(1);
    });
 
    if let Err(err) = minigrep:: run(config){
        eprintln!("app run err: {err}");
        std::process::exit(1);
    };
}