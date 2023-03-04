use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}, {}> Line: {}",
            self.token_type, self.lexeme, self.line
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    // Operators
    Plus,
    Minus,
    Times,
    Divide,
    Equals,
    DoubleEquals,
    NotEquals,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,

    // Special characters
    Semicolon,
    OpeningParentheses,
    ClosingParentheses,
    OpeningBraces,
    ClosingBraces,

    // Keywords
    True,
    False,
    And,
    Or,
    Var,
    Print,
    If,
    Else,
    While,

    // Literals
    Number,
    String,

    // Variables
    Identifier,

    // Returned once when whole input file is tokenized.
    EndOfile,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We delegate to its derived debug form, as that one returns the enum's name as a string -
        // which is what we want.
        write!(f, "{:?}", self)
    }
}
