use std::io::{self, Write};

use logos::Logos;
use crate::lexer;
use crate::parser::Parser;

fn exec(code: String) {
    let mut tokens = lexer::SyntaxKind
        ::lexer(&code)
        .collect::<Vec<lexer::SyntaxKind>>();
    println!("{:?}", tokens);
}

pub fn repl() {
    println!("Lari v{} on `{}` [{}]", env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    loop {
        write!(stdout, "\n>>> ");
        stdout.flush();
        stdin.read_line(&mut input);

        let parse = Parser::new(&input).parse();
        println!("{}", parse.debug_tree());

        input.clear();
        // exec(input);
    }
}