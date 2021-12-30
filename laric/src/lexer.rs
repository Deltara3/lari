use logos::Logos;

fn conv_str(string: &str) -> String {
    string
        .replace("\r", "")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\\"", "\"")
        .replace("\\'", "'")
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Type
    #[regex(r"([0-9]+(\.[0-9]*)?|\.[0-9]*)", |lex| lex.slice().parse::<u64>())]
    Int(u64),

    #[regex(r#""(?:\\.|[^\\"])*"|'(?:\\.|[^\\'])*'"#, |string| conv_str(&string.slice()[1..string.slice().len()-1]))]
    Str(String),

    #[regex(r"[a-zA-Z_ඞ][a-zA-Z_0-9ඞ]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[token("@")]
    TypeIdent,

    // Operator
    #[token("+=")]
    IncEq,
    #[token("-=")]
    DecEq,
    #[token("*=")]
    MulEq,
    #[token("/=")]
    DivEq,
    #[token("%=")]
    ModEq,
    #[token("^=")]
    PowEq,
    #[token("+")]
    Inc,
    #[token("++")]
    DblInc,
    #[token("-")]
    Dec,
    #[token("--")]
    DblDec,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,
    #[token("^")]
    Pow,
    #[token("<=")]
    LessEq,
    #[token(">=")]
    MoreEq,
    #[token("<")]
    Less,
    #[token(">")]
    More,
    #[token("!=")]
    NotEq,
    #[token("!")]
    Not,
    #[token(":")]
    Colon,
    #[token("::")]
    DblColon,
    #[token(";")]
    SemiColon,
    #[token("==")]
    Eq,
    // #[token("===")]
    // CompEq,
    #[token("=")]
    Assign,
    #[token("||")]
    Or,
    #[token("|")]
    Pipe,
    #[token("&&")]
    And,
    #[token("..")]
    Range,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("->")]
    ArrR,
    #[token("<-")]
    ArrL,
    #[token("<->")]
    ArrB,
    #[token("=>")]
    DblArrR,
    #[token("<=>")]
    DblArrB,
    #[token("~>")]
    SqArrR,
    #[token("<~")]
    SqArrL,
    #[token("<~>")]
    SqArrB,

    // Keyword
    #[token("fn")]
    Func,
    #[token("use")]
    Use,
    #[token("pub")]
    Pub,

    // Control
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("while")]
    While,

    // Other
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrace,
    #[token("]")]
    RBrace,
    #[token("{")]
    LBrack,
    #[token("}")]
    RBrack,

    // Actually other
    #[token("🎷🐛")]
    Sax,

    // Error
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Err,

    // File
    #[regex(r"\r\n|\r|\n")]
    Eol,
    Eof
}