use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Import,
    Return,
    Function,
    SquigglyArrow,
    Arrow,
    Semi,
    Colon,
    Comma,
    LCurly,
    RCurly,
    LParen,
    RParen,
    Word { word: String },
    Type { name: String },
}

pub fn tokenize(code: &str) -> Vec<Token> {
    // TODO the actual tokenizer
    // TODO return Vec<Token>
    let mut tokens: Vec<Token> = vec![];

    // load in regexes
    // TODO add regexes for the rest of what you wanna add
    let line_comment = Regex::new(r"(?m)/{2,} .*$").unwrap(); // match anything that starts with // and beyond until \n
                                                              // strip comments
    let code_w_stripped_comments = line_comment.replace_all(code, "");

    let mut code_no_newline = String::from("");

    for line in code_w_stripped_comments.lines() {
        code_no_newline.push_str(line);
    }

    //println!("{}", code_no_newline);

    for statement in code_no_newline.split(";") {
        for mut word in statement.split(" ") {
            println!("{}", word);
            if word.starts_with("{") {
                tokens.push(Token::LCurly);
                word = &word[1..];
            }
            if word.starts_with("}") {
                tokens.push(Token::RCurly);
                word = &word[1..];
            }
            if word.ends_with("{") {
                word = &word[..word.len() - 1];
                tokens.push(Token::Word {
                    word: word.to_string(),
                });
                tokens.push(Token::LCurly);
                word = "";
            }
            if word.ends_with("}") {
                word = &word[..word.len() - 1];
                tokens.push(Token::Word {
                    word: word.to_string(),
                });
                tokens.push(Token::RCurly);
                word = "";
            }
            if word.starts_with(":") {
                tokens.push(Token::Colon);
                word = &word[1..];
            }
            if word.ends_with(":") {
                word = &word[..word.len() - 1];
                tokens.push(Token::Word {
                    word: word.to_string(),
                });
                tokens.push(Token::Colon);
                word = "";
            }
            if word.starts_with("(") {
                tokens.push(Token::LParen);
                word = &word[1..];
            }
            if word.starts_with(")") {
                tokens.push(Token::RParen);
                word = &word[1..];
            }
            if word.ends_with(")") {
                word = &word[..word.len() - 1];
                tokens.push(Token::Word {
                    word: word.to_string(),
                });
                tokens.push(Token::RParen);
                word = "";
            }
            if word.ends_with("(") {
                word = &word[..word.len() - 1];
                tokens.push(Token::Word {
                    word: word.to_string(),
                });
                tokens.push(Token::LParen);
                word = "";
            }
            if word.starts_with(",") {
                tokens.push(Token::Comma);
                word = &word[1..];
            }
            if word.ends_with(",") {
                word = &word[..word.len() - 2];
                tokens.push(Token::Word {
                    word: word.to_string(),
                });
                tokens.push(Token::Comma);
                word = "";
            }
            if word.starts_with("@") {
                tokens.push(Token::Type {
                    name: (&word[1..]).to_string(),
                });
                word = "";
            }

            if word != "" {
                match word {
                    "fn" => tokens.push(Token::Function),
                    "return" => tokens.push(Token::Return),
                    "import" => tokens.push(Token::Import),
                    "->" => tokens.push(Token::Arrow),
                    "~>" => tokens.push(Token::SquigglyArrow),
                    _ => {
                        tokens.push(Token::Word {
                            word: word.to_string(),
                        });
                    }
                }
            }
        }
        tokens.push(Token::Semi);
    }
    return tokens;
}
