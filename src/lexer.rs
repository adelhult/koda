use logos::Logos;

/// Lex the code into Tokens
pub fn lex(code: &str) -> Vec<Token> {
    let lex = Token::lexer(&code);
    lex.collect()
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("och")]
    And,
    #[token("bryt")]
    Break,
    #[token("gör")]
    Do,
    #[token("annars")]
    Else,
    #[token("annarsom")]
    Elseif,
    #[token("slut")]
    End,
    #[token("falskt")]
    False,
    #[token("för")]
    For,
    #[token("funktion")]
    Function,
    #[token("om")]
    If,
    #[token("i")]
    In,
    #[token("lokal")]
    Local,
    #[token("inte")]
    Not,
    #[token("eller")]
    Or,
    #[token("upprepa")]
    Repeat,
    #[token("ge")]
    Return,
    #[token("sant")]
    True,
    #[token("tills")]
    Until,
    #[token("medan")]
    While,
    #[token("ingenting")]
    Nil,
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token("=")]
    AssignmentOperator,
    #[token("..")]
    Concat,
    #[token("...")]
    VarArgs,
    #[token("#")]
    LengthOperator,
    #[token("==")]
    Equal,
    #[token("~=")] 
    NotEqual,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token(">=")]
    GreaterOrEqual,
    #[token("<=")]
    LessOrEqual,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulus,
    #[token("+")]
    Add,
    #[token("-")]
    Subtract,
    #[token("^")]
    Exponent,
    #[regex(r"--\[\[[^\]]*\]\]|--.*")]
    Comment,
    #[regex(r#""[^"\\]*(?:\\.[^"\\]*)*""#, |lex| lex.slice().parse())]
    #[regex(r#"'[^'\\]*(?:\\.[^'\\]*)*'"#, |lex| lex.slice().parse())]
    Str(String),
    #[regex(r"[a-zA-ZåäöÅÄÖ_-]+\d*[a-zA-ZåäöÅÄÖ_-]*", |lex| lex.slice().parse(), priority=1)]
    Ident(String),
    // note, .5 is not supported
    #[regex(r"\d+\.?[\d]*", |lex| lex.slice().parse())]
    Number(String),
    #[error]
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Error
}