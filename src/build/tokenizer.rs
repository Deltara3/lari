#[derive(Debug)]
pub enum Token {
    Arrow,
    Function,
    Colon,
    Import,
    FunnyArrow,
    Name {
        name: String,
    },
    Return,
    OpeningCurlyBracket,
    ClosingCurlyBracket,
    Comment,
    ClosingComment,
    Newline,
    Semi,
}

pub fn tokenize(code: &str) -> Vec<Token> {
    // TODO the actual tokenizer
    // TODO return Vec<Token>
    let mut tokens: Vec<Token> = vec![];
    // TODO fix the fact that comments need spaces to work
    let in_comment = 0; // 0: not in comment, 1: in line comment, 2: in block comment
    for word in code.split(" ") {
        tokens.push(match word {
            "->" => Token::Arrow,
            "~>" => Token::FunnyArrow,
            "fn" => Token::Function,
            "import" => Token::Import,
            ":" => Token::Colon,
            "return" => Token::Return,
            "{" => Token::OpeningCurlyBracket,
            "}" => Token::ClosingCurlyBracket,
            "//" => Token::Comment,
            "/*" => Token::Comment,
            "*/" => Token::ClosingComment,
            "\n" => Token::Newline,
            ";" =>Token::Semi,
            _ => Token::Name {
                name: word.to_string()
            },
        });
    }
    return tokens; 
}