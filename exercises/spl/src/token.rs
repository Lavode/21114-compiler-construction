use std::fmt::Display;

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

#[derive(Debug)]
pub enum TokenType {
    // Operators
    Plus,
    Minus,
    Times,
    Divide,
    Equals,
    DoubleEquals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEqualThan,
    LessOrEqualThan,

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
    Identifiers,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We delegate to its derived debug form, as that one returns the enum's name as a string -
        // which is what we want.
        write!(f, "{:?}", self)
    }
}
