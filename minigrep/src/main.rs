
// convention is to use struct directly and use the crate or module name for function
use minigrep::Config; 
use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // minigrep is top level path like std so that we don't need to use `use minigerep`
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:{e}");
        process::exit(1)
    }
}
