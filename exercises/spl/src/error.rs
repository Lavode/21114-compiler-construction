use std::fmt::Display;

/// Position within an input file
#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Errors returned by Lexer
#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    /// Returned when the lexer encounterd an unterminated string sequence.
    UnterminatedStringSequence {
        starts_at: Position,
        ends_at: Position,
    },

    /// Returned when the lexer encountered an unexpected character.
    UnexpectedChar { position: Position, c: char },
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnterminatedStringSequence { starts_at, ends_at } => {
                write!(
                    f,
                    "Unterminted string sequence found, starting at {}, ending at {}",
                    starts_at, ends_at,
                )
            }
            LexerError::UnexpectedChar { position, c } => {
                write!(
                    f,
                    "Unexpected char `{}` (unicode {}) found at {}",
                    c,
                    c.escape_unicode(),
                    position
                )
            }
        }
    }
}
