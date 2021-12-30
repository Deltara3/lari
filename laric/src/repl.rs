use std::io::{self, Write};

use logos::Logos;
use crate::lexer;

fn exec(code: String) {
    let mut tokens = lexer::Token
        ::lexer(&code)
        .collect::<Vec<lexer::Token>>();
    println!("{:?}", tokens);
}

pub fn repl() {
    println!("Lari v{} on `{}` [{}]", env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH);
    loop {
        print!("{}", "\n>>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin?");
        exec(input);
    }
}