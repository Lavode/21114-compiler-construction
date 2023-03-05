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

    /// Peek at the next character without advancing the position in the input.
    ///
    /// Returns None if the end of the input is reached.
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Advance by one character, returning it.
    ///
    /// Returns None if the end of the input is reached.
    fn advance(&mut self) -> Option<char> {
        self.column += 1;

        let next = self.chars.next();

        if next.is_some() {
            if next.unwrap() == '\n' {
                self.line += 1;
                self.column = 0;
            }
        }

        next
    }

    /// Advance if the next character is equal to `expected`.
    fn advance_if_equal(&mut self, expected: char) -> bool {
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

    /// Advance until the next character is equal to `expected`.
    ///
    /// Returns a vector of all characters through which the lexer advanced. `expected` is not part
    /// of the output vector.
    ///
    /// Returns an error if the lexer ran out of input before finding a match.
    fn advance_until_equal(&mut self, expected: char) -> Result<Vec<char>, ()> {
        let mut out = Vec::new();

        loop {
            match self.advance() {
                Some(c) => {
                    if c == expected {
                        break;
                    }

                    out.push(c);
                }

                // We reached the end of the input without ifnding our expected character.
                None => return Err(()),
            }
        }

        return Ok(out);
    }

    /// Advance as long as the provided closure evaluates to true for the next character.
    ///
    /// Returns a vector of all characters through which the lexer advanced.
    fn advance_while_matching<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut out = Vec::new();

        while let Some(c) = self.peek() {
            if !f(*c) {
                break;
            }

            // We know that something is there as peek() returned Some, so we unwrap.
            out.push(self.advance().unwrap());
        }

        return out;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.advance() {
            match c {
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

                '/' => {
                    if self.advance_if_equal('/') {
                        // Line comment
                        let _ = self.advance_until_equal('\n');
                    } else {
                        // Divides operator
                        tokens.push(Token {
                            token_type: TokenType::Divide,
                            lexeme: "/".into(),
                            line: self.line,
                        });
                    }
                }

                '=' => {
                    if self.advance_if_equal('=') {
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
                    if self.advance_if_equal('=') {
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
                    if self.advance_if_equal('=') {
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
                    if self.advance_if_equal('=') {
                        tokens.push(Token {
                            token_type: TokenType::NotEquals,
                            lexeme: "!=".into(),
                            line: self.line,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::BooleanNot,
                            lexeme: "!".into(),
                            line: self.line,
                        });
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

                '"' => match self.advance_until_equal('"') {
                    Ok(chars) => tokens.push(Token {
                        token_type: TokenType::String,
                        lexeme: String::from_iter(chars.iter()),
                        line: self.line,
                    }),
                    Err(_) => eprintln!(
                        "Error on line {}, column {}: Found unterminated string sequence.",
                        self.line, self.column
                    ),
                },

                // advance() handles line and column numbers, there's naught for us to do but
                // enjoy this fleeting moment of quiet.
                '\n' => {}

                // Whitespace is silently consumed
                ' ' | '\t' => {}

                _ => {
                    if c.is_alphabetic() {
                        let mut name = String::new();
                        name.push(c);

                        // Consume all following alphanumeric characters
                        let additional_chars = self.advance_while_matching(|c| c.is_alphanumeric());
                        name.extend(additional_chars.iter());

                        // Keywords take precedence over identifiers
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
                                });
                            }
                        }
                    } else if c.is_digit(10) {
                        let mut number = String::new();
                        number.push(c);

                        // Consume all digits before the decimal point.
                        let additional_digits = self.advance_while_matching(|c| c.is_digit(10));
                        number.extend(additional_digits.iter());

                        // Consume decimal digits if present
                        if self.advance_if_equal('.') {
                            number.push('.');
                            let additional_digits = self.advance_while_matching(|c| c.is_digit(10));
                            number.extend(additional_digits.iter());
                        }

                        tokens.push(Token {
                            token_type: TokenType::Number,
                            lexeme: number,
                            line: self.line,
                        });
                    } else {
                        eprintln!(
                            "Error on line {}, column {}: Found unexpected char '{}' (Unicode {})",
                            self.line,
                            self.column,
                            c,
                            c.escape_unicode()
                        );
                    }
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
        assert_eq!(lex.line, 1);
        assert_eq!(lex.column, 1);

        assert_eq!(lex.advance().unwrap(), 'o');
        assert_eq!(lex.line, 1);
        assert_eq!(lex.column, 2);

        assert_eq!(lex.advance().unwrap(), 'o');
        assert_eq!(lex.line, 1);
        assert_eq!(lex.column, 3);

        // At end of input
        let mut lex = Lexer::new("");
        assert!(lex.advance().is_none());
    }

    #[test]
    fn test_advance_past_newline() {
        let mut lex = Lexer::new("ab\na");
        // 'a'
        lex.advance().unwrap();
        // 'b'
        lex.advance().unwrap();
        // '\n'
        lex.advance().unwrap();
        assert_eq!(lex.line, 2);
        assert_eq!(lex.column, 0);
    }

    #[test]
    fn test_advance_if_equal() {
        let mut lex = Lexer::new("foo");

        assert_eq!(lex.advance_if_equal('f'), true);
        assert_eq!(lex.column, 1);

        assert_eq!(lex.advance_if_equal('f'), false);
        assert_eq!(lex.column, 1);

        assert_eq!(lex.advance_if_equal('o'), true);
        assert_eq!(lex.column, 2);

        // At end of input
        let mut lex = Lexer::new("");
        assert!(!lex.advance_if_equal('f'));
    }

    #[test]
    fn test_advance_until_equal() {
        let mut lex = Lexer::new("abc|def");
        let tokens = lex.advance_until_equal('|');
        assert_eq!(tokens.unwrap(), vec!['a', 'b', 'c']);
        assert_eq!(lex.column, 4);
        assert_eq!(*lex.peek().unwrap(), 'd');

        // At end of input
        let mut lex = Lexer::new("abc|");
        let tokens = lex.advance_until_equal('|');
        assert_eq!(tokens.unwrap(), vec!['a', 'b', 'c']);
        assert_eq!(lex.column, 4);
        assert!(lex.peek().is_none());
    }

    #[test]
    fn test_advance_until_equal_no_match() {
        let mut lex = Lexer::new("abc");
        let tokens = lex.advance_until_equal('|');
        assert!(tokens.is_err());
        assert_eq!(lex.column, 4);
        assert!(lex.peek().is_none());
    }

    #[test]
    fn test_advance_while_matching() {
        let mut lex = Lexer::new("abc123def");
        let tokens = lex.advance_while_matching(|c| c.is_alphabetic());
        assert_eq!(tokens, vec!['a', 'b', 'c']);
        assert_eq!(lex.column, 3);

        let mut lex = Lexer::new("abc123def");
        let tokens = lex.advance_while_matching(|c| c.is_alphanumeric());
        assert_eq!(tokens, vec!['a', 'b', 'c', '1', '2', '3', 'd', 'e', 'f']);
        assert_eq!(lex.column, 9);
    }

    #[test]
    fn test_advance_while_matching_no_match() {
        let mut lex = Lexer::new("-0abc123def");
        let tokens = lex.advance_while_matching(|c| c.is_alphanumeric());
        assert_eq!(tokens, vec![]);
        assert_eq!(lex.column, 0);
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
    fn test_boolean_not() {
        let mut lex = Lexer::new("!");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::BooleanNot,
                lexeme: "!".into(),
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
    fn test_false() {
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

    #[test]
    fn test_number() {
        // Integer
        let mut lex = Lexer::new("123");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Number,
                lexeme: "123".into(),
                line: 1
            }
        );

        // Float
        let mut lex = Lexer::new("123.456");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Number,
                lexeme: "123.456".into(),
                line: 1
            }
        );

        // Float with no decimal digits.
        let mut lex = Lexer::new("123.");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Number,
                lexeme: "123.".into(),
                line: 1
            }
        );

        // Float with no multiple decimal points.
        // Lexer should recogniez the number (123.456) fine, but then balk on finding a lone
        // decimal point.
        let mut lex = Lexer::new("123.456.");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Number,
                lexeme: "123.456".into(),
                line: 1
            }
        );
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn test_string() {
        let mut lex = Lexer::new("\"Hello world\"");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::String,
                lexeme: "Hello world".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_empty_string() {
        let mut lex = Lexer::new("\"\"");
        let tokens = lex.tokenize();
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::String,
                lexeme: "".into(),
                line: 1
            }
        );
    }

    #[test]
    fn test_unterminated_string() {
        let mut lex = Lexer::new("\"Hello world");
        let tokens = lex.tokenize();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_comment() {
        let mut lex = Lexer::new("// This is a comment\n1");
        let tokens = lex.tokenize();

        // Should have outright skipped the comment
        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Number,
                lexeme: "1".into(),
                line: 2
            }
        );

        assert_eq!(lex.line, 2);
        // We already consumed the single digit on line two, so are in column two now. (Which
        // happens to be the EOF)
        assert_eq!(lex.column, 2);
    }

    #[test]
    fn test_newline() {
        let mut lex = Lexer::new("a = 1;\nb = 2;");
        let _ = lex.tokenize();

        assert_eq!(lex.line, 2);
        assert_eq!(lex.column, 7);
    }

    #[test]
    fn test_tokenize() {
        let input = "
var b = true ; // A boolean
var i = 123; // A number
var d = 12.3; // Another number
var s = \"123 \"; // This is a string , not a number

i + d; // 135.3
1 == 2; // false
!true; // false
true or false; // true
var average = (min + max ) / 2;

{
	print \"Hello , world !\";
	print \"Hello , SPL Prime world !\";
}

if ( i == s ) {
	print \"yes\";
} else {
	print \"no\";
}

var a = 1;
while (a < 10) {
	print a;
	a = a + 1;
}
";

        let mut lex = Lexer::new(input);
        let tokens = lex.tokenize();

        assert_eq!(tokens.len(), 93);
    }
}
