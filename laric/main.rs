mod build;
use build::build;

use std::env;

use std::process::exit;

fn main() {
    let argv: Vec<String> = env::args().collect();

    match argv[1].as_str() {
        "b" | "build" => build(argv),
        _ => {println!("unknown command"); exit(0)},
    }
}