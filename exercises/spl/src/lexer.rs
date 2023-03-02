use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            chars: source.chars().peekable(),
            line: 0,
        }
    }

    pub fn peek(&mut self) -> char {
        // Dereferencing will lead to a copy of the char, but given it's a single byte we can live
        // with that.
        *self.chars.peek().unwrap()
    }

    pub fn advance(&mut self) -> char {
        self.chars.next().unwrap()
    }

    pub fn advance_if_matching(&mut self, expected: char) -> bool {
        if self.peek() == expected {
            self.chars.next();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek() {
        let mut lex = Lexer::new("foo");

        assert_eq!(lex.peek(), 'f');
        // Should not have advanced
        assert_eq!(lex.peek(), 'f');
    }

    #[test]
    fn test_advance() {
        let mut lex = Lexer::new("foo");

        assert_eq!(lex.advance(), 'f');
        assert_eq!(lex.advance(), 'o');
        assert_eq!(lex.advance(), 'o');
    }

    #[test]
    fn test_advance_if_matching() {
        let mut lex = Lexer::new("foo");

        assert_eq!(lex.advance_if_matching('f'), true);
        assert_eq!(lex.advance_if_matching('f'), false);
        assert_eq!(lex.advance_if_matching('o'), true);
    }
}
