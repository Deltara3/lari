mod lexer;

use clap::{Arg, App, SubCommand};

fn main() {
    // Argument parser
    let matches =
    App::new("laric")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Lari compiler")
        .arg(Arg::with_name("repl")
            .long("repl")
            .help("Enters the Lari repl")
            .takes_value(false))
        .arg(Arg::with_name("input")
            .short("i")
            .long("in")
            .value_name("FILE")
            .help("Input file")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .value_name("FILE")
            .help("Output file")
            .takes_value(true))
        .get_matches();
}
