use std::io::{self, Write};
use crate::parser::Parser;

pub fn repl() -> io::Result<()> {
    println!("Lari v{} on `{}` [{}]", env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    loop {
        write!(stdout, "\n>>> ")?;
        stdout.flush()?;
        stdin.read_line(&mut input)?;

        let parse = Parser::new(&input).parse();
        println!("{}", parse.debug_tree());

        input.clear();
    }
}