use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Logos, FromPrimitive, ToPrimitive)]
pub(crate) enum SyntaxKind {
    // Type
    /* Temporarily commented out.
    #[regex(r#""(?:\\.|[^\\"])*"|'(?:\\.|[^\\'])*'"#, |string| conv_str(&string.slice()[1..string.slice().len()-1]))]
    Str(String),
    */

    #[token("@")]
    TypeIdent,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,
    #[regex("[0-9]+")]
    Int,

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
    Add,
    #[token("++")]
    Inc,
    #[token("-")]
    Min,
    #[token("--")]
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
    #[token("::")]
    DblColon,
    #[token(";")]
    SemiColon,
    #[token("==")]
    Eq,
    // #[token("===")]
    // TypeEq,
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
    ReturnType,
    #[token("<-")]
    Return,
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
    #[token("macro")]
    Macro,
    #[token("use")]
    Use,
    #[token("pub")]
    Pub,
    #[token("let")]
    Let,
    #[token("mut")]
    Mutable,

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

    Root,
    BinOp,
    PreExpr,

    // File
    #[regex(r"\r\n|\r|\n")]
    Eol,
    Eof
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some((kind, text))
    }
}