use regex::Regex;

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

    // load in regexes
    // TODO add regexes for the rest of what you wanna add
    let line_comment = Regex::new(r"(?m)/{2,} .*$").unwrap(); // match anything that starts with // and beyond

    // strip comments
    let code_w_stripped_comments = line_comment.replace_all(code, " ");

    println!("{}", code_w_stripped_comments);

    return tokens; 
}