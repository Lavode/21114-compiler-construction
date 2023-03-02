use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            chars: source.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.column += 1;
        self.chars.next()
    }

    fn advance_if_matching(&mut self, expected: char) -> bool {
        match self.peek() {
            None => return false,
            Some(c) => {
                if *c == expected {
                    self.chars.next();
                    self.column += 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match self.advance() {
                Some(c) => match c {
                    '+' => tokens.push(Token {
                        token_type: TokenType::Plus,
                        lexeme: "+".into(),
                        line: self.line,
                    }),

                    '-' => tokens.push(Token {
                        token_type: TokenType::Minus,
                        lexeme: "-".into(),
                        line: self.line,
                    }),

                    '*' => tokens.push(Token {
                        token_type: TokenType::Times,
                        lexeme: "*".into(),
                        line: self.line,
                    }),

                    '/' => tokens.push(Token {
                        token_type: TokenType::Divide,
                        lexeme: "/".into(),
                        line: self.line,
                    }),

                    '=' => {
                        if self.advance_if_matching('=') {
                            tokens.push(Token {
                                token_type: TokenType::DoubleEquals,
                                lexeme: "==".into(),
                                line: self.line,
                            });
                        } else {
                            tokens.push(Token {
                                token_type: TokenType::Equals,
                                lexeme: "=".into(),
                                line: self.line,
                            });
                        }
                    }

                    '>' => {
                        if self.advance_if_matching('=') {
                            tokens.push(Token {
                                token_type: TokenType::GreaterOrEqual,
                                lexeme: ">=".into(),
                                line: self.line,
                            });
                        } else {
                            tokens.push(Token {
                                token_type: TokenType::Greater,
                                lexeme: ">".into(),
                                line: self.line,
                            });
                        }
                    }

                    '<' => {
                        if self.advance_if_matching('=') {
                            tokens.push(Token {
                                token_type: TokenType::LessOrEqual,
                                lexeme: "<=".into(),
                                line: self.line,
                            });
                        } else {
                            tokens.push(Token {
                                token_type: TokenType::Less,
                                lexeme: "<".into(),
                                line: self.line,
                            });
                        }
                    }

                    '!' => {
                        if self.advance_if_matching('=') {
                            tokens.push(Token {
                                token_type: TokenType::NotEquals,
                                lexeme: "!=".into(),
                                line: self.line,
                            });
                        } else {
                            // We found a `!` not followed by an `=`, which is a lexer error. Log
                            // it, and consume the incorrect character, to carry on.
                            match self.advance() {
                                Some(wrong_char) => eprintln!(
                                    "Error on line {}, column {}: Found `!` followed by `{}` rather than `=`",
                                    self.line, self.column, wrong_char
                                ),
                                None => eprintln!(
                                    "Error on line {}, column {}: Lone `!` found at end of input",
                                    self.line, self.column,
                                ),
                            }
                        }
                    }

                    ';' => tokens.push(Token {
                        token_type: TokenType::Semicolon,
                        lexeme: ";".into(),
                        line: self.line,
                    }),

                    '(' => tokens.push(Token {
                        token_type: TokenType::OpeningParentheses,
                        lexeme: "(".into(),
                        line: self.line,
                    }),
                    ')' => tokens.push(Token {
                        token_type: TokenType::ClosingParentheses,
                        lexeme: ")".into(),
                        line: self.line,
                    }),

                    '{' => tokens.push(Token {
                        token_type: TokenType::OpeningBraces,
                        lexeme: "{".into(),
                        line: self.line,
                    }),
                    '}' => tokens.push(Token {
                        token_type: TokenType::ClosingBraces,
                        lexeme: "}".into(),
                        line: self.line,
                    }),

                    _ => {
                        if c.is_alphabetic() {
                            let mut name = String::new();
                            name.push(c);

                            // TODO: Extract this into utility method.
                            // Consume all following alphanumeric characters, or until the end of
                            // the input.
                            loop {
                                match self.peek() {
                                    Some(next_char) => {
                                        if next_char.is_alphanumeric() {
                                            // We can unwrap here as we know that there is one
                                            // present
                                            name.push(self.advance().unwrap());
                                        } else {
                                            break;
                                        }
                                    }
                                    None => break,
                                }
                            }

                            match name.as_str() {
                                "true" => tokens.push(Token {
                                    token_type: TokenType::True,
                                    lexeme: "true".into(),
                                    line: self.line,
                                }),

                                "false" => tokens.push(Token {
                                    token_type: TokenType::False,
                                    lexeme: "false".into(),
                                    line: self.line,
                                }),

                                "and" => tokens.push(Token {
                                    token_type: TokenType::And,
                                    lexeme: "and".into(),
                                    line: self.line,
                                }),

                                "or" => tokens.push(Token {
                                    token_type: TokenType::Or,
                                    lexeme: "or".into(),
                                    line: self.line,
                                }),

                                "var" => tokens.push(Token {
                                    token_type: TokenType::Var,
                                    lexeme: "var".into(),
                                    line: self.line,
                                }),

                                "print" => tokens.push(Token {
                                    token_type: TokenType::Print,
                                    lexeme: "print".into(),
                                    line: self.line,
                                }),

                                "if" => tokens.push(Token {
                                    token_type: TokenType::If,
                                    lexeme: "if".into(),
                                    line: self.line,
                                }),

                                "else" => tokens.push(Token {
                                    token_type: TokenType::Else,
                                    lexeme: "else".into(),
                                    line: self.line,
                                }),

                                "while" => tokens.push(Token {
                                    token_type: TokenType::While,
                                    lexeme: "while".into(),
                                    line: self.line,
                                }),

                                _ => {
                                    // An alphanumeric name which doesn't correspond to any
                                    // keyword is an identifier.
                                    tokens.push(Token {
                                        token_type: TokenType::Identifier,
                                        lexeme: name,
                                        line: self.line,
                                    })
                                }
                            }
                        } else {
                            println!("Other char found: {}", c);
                        }
                    }
                },
                None => {
                    // We reached the end of our input
                    break;
                }
            }
        }

        return tokens;
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn test_peek() {
        let mut lex = Lexer::new("foo");

        assert_eq!(*lex.peek().unwrap(), 'f');

        // Should not have advanced
        assert_eq!(*lex.peek().unwrap(), 'f');

        // At end of input
        let mut lex = Lexer::new("");
        assert!(lex.peek().is_none());
    }

    #[test]
    fn test_advance() {
        let mut lex = Lexer::new("foo");

        assert_eq!(lex.advance().unwrap(), 'f');
        assert_eq!(lex.column, 1);

        assert_eq!(lex.advance().unwrap(), 'o');
        assert_eq!(lex.column, 2);

        assert_eq!(lex.advance().unwrap(), 'o');
        assert_eq!(lex.column, 3);

        // At end of input
        let mut lex = Lexer::new("");
        assert!(lex.advance().is_none());
    }

    #[test]
    fn test_advance_if_matching() {
        let mut lex = Lexer::new("foo");

        assert_eq!(lex.advance_if_matching('f'), true);
        assert_eq!(lex.column, 1);

        assert_eq!(lex.advance_if_matching('f'), false);
        assert_eq!(lex.column, 1);

        assert_eq!(lex.advance_if_matching('o'), true);
        assert_eq!(lex.column, 2);

        // At end of input
        let mut lex = Lexer::new("");
        assert!(!lex.advance_if_matching('f'));
    }

    #[test]
    fn test_plus() {
        let mut lex = Lexer::new("+");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Plus,
                lexeme: "+".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_minus() {
        let mut lex = Lexer::new("-");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_times() {
        let mut lex = Lexer::new("*");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Times,
                lexeme: "*".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_divide() {
        let mut lex = Lexer::new("/");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Divide,
                lexeme: "/".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_equals() {
        let mut lex = Lexer::new("=");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Equals,
                lexeme: "=".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_double_equals() {
        let mut lex = Lexer::new("==");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::DoubleEquals,
                lexeme: "==".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_not_equals() {
        let mut lex = Lexer::new("!=");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::NotEquals,
                lexeme: "!=".into(),
                line: 1
            }
        );

        // TODO test error
        // `!` followed by char other than `=`
        let mut lex = Lexer::new("!a");
        let tokens = lex.tokenize();
        assert_eq!(tokens.len(), 0);
    }
    #[test]
    fn test_greater_than() {
        let mut lex = Lexer::new(">");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Greater,
                lexeme: ">".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_less_than() {
        let mut lex = Lexer::new("<");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Less,
                lexeme: "<".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_greater_or_equal() {
        let mut lex = Lexer::new(">=");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::GreaterOrEqual,
                lexeme: ">=".into(),
                line: 1
            }
        );
    }
    #[test]
    fn test_less_or_equal() {
        let mut lex = Lexer::new("<=");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::LessOrEqual,
                lexeme: "<=".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_semicolon() {
        let mut lex = Lexer::new(";");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_opening_parentheses() {
        let mut lex = Lexer::new("(");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::OpeningParentheses,
                lexeme: "(".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_closing_parentheses() {
        let mut lex = Lexer::new(")");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::ClosingParentheses,
                lexeme: ")".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_opening_braces() {
        let mut lex = Lexer::new("{");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::OpeningBraces,
                lexeme: "{".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_closing_braces() {
        let mut lex = Lexer::new("}");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::ClosingBraces,
                lexeme: "}".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_true() {
        let mut lex = Lexer::new("true");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::True,
                lexeme: "true".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_() {
        let mut lex = Lexer::new("false");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::False,
                lexeme: "false".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_and() {
        let mut lex = Lexer::new("and");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::And,
                lexeme: "and".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_or() {
        let mut lex = Lexer::new("or");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Or,
                lexeme: "or".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_var() {
        let mut lex = Lexer::new("var");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Var,
                lexeme: "var".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_print() {
        let mut lex = Lexer::new("print");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Print,
                lexeme: "print".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_if() {
        let mut lex = Lexer::new("if");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::If,
                lexeme: "if".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_else() {
        let mut lex = Lexer::new("else");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Else,
                lexeme: "else".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_while() {
        let mut lex = Lexer::new("while");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::While,
                lexeme: "while".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_identifier() {
        let mut lex = Lexer::new("foo");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Identifier,
                lexeme: "foo".into(),
                line: 1
            }
        );

        // Starting with a keyword
        let mut lex = Lexer::new("if32");
        let tokens = lex.tokenize();

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Identifier,
                lexeme: "if32".into(),
                line: 1
            }
        );
    }

    // TODO: Test
    // - Numbers
    // - Strings
    // - Comments
}
