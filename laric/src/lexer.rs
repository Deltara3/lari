use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Type

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
    #[token("-")]
    Dec,
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
    #[token("==")]
    Eq,
    #[token("===")]
    CompEq,
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
    #[token("import")]
    Import,

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

    // Error
    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,

    // File
    #[regex(r";|\n|\r")]
    Eol,
    Eof
}