use minigrep::args::Args;
use minigrep::func::run;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments = Args::build(&args).unwrap_or_else(|e| {
        println!("Error: {e}");
        exit(1);
    });

    if let Err(e) = run(arguments) {
        println!("Error: {e}");
    }
}
