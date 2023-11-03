use crate::current_iterator::CurrentIterator;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    SimpleToken(SimpleToken),
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(u64),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SimpleToken {
    Let,
    Mut,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
    Assignment,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}

#[derive(Clone, Copy)]
struct SimpleTokenMatcher {
    token: SimpleToken,
    match_str: &'static str,
    is_word: bool,
}

pub struct Lexer<'a> {
    text: CurrentIterator<Chars<'a>>,
    match_tokens: Vec<SimpleTokenMatcher>,
}

impl Lexer<'_> {
    pub fn new<'a>(input_data: &'a str) -> Lexer<'a> {
        Lexer {
            text: CurrentIterator::new(input_data.chars()),
            match_tokens: vec![
                SimpleTokenMatcher {
                    token: SimpleToken::Let,
                    match_str: "let",
                    is_word: true,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::Mut,
                    match_str: "mut",
                    is_word: true,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::Exponent,
                    match_str: "**",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::Addition,
                    match_str: "+",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::Subtraction,
                    match_str: "-",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::Multiplication,
                    match_str: "*",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::Division,
                    match_str: "/",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::Assignment,
                    match_str: "=",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::Semicolon,
                    match_str: ";",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::LParen,
                    match_str: "(",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::RParen,
                    match_str: ")",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::LBrace,
                    match_str: "{",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::RBrace,
                    match_str: "}",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::LBracket,
                    match_str: "[",
                    is_word: false,
                },
                SimpleTokenMatcher {
                    token: SimpleToken::RBracket,
                    match_str: "]",
                    is_word: false,
                },
            ],
        }
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = vec![];

        while !self.consume_whitespace()
            && let Some(token) = self.get_next_token()?
        {
            tokens.push(token);
        }

        self.consume_whitespace();

        // If there is more text, an invalid token was encountered.
        let mut it = self.text.clone();
        if it.next().is_some() {
            return Err(LexerError::InvalidToken);
        }

        Ok(tokens)
    }

    // Returns true iff EOF.
    fn consume_whitespace(&mut self) -> bool {
        let mut it = self.text.clone();
        let mut i: usize = 0;

        loop {
            i += 1;

            match it.next() {
                None => {
                    self.text.advance_by(i - 1).unwrap();
                    return true;
                }
                Some(ch) => {
                    if !ch.is_whitespace() {
                        break;
                    }
                }
            }
        }

        self.text.advance_by(i - 1).unwrap();
        return false;
    }

    fn get_next_token_simple(&mut self) -> Result<Option<SimpleToken>, LexerError> {
        'check_next_token: for &token_matcher in &self.match_tokens {
            let mut it = self.text.clone();
            let prev = it.current();

            if token_matcher.is_word
                && !prev.is_none()
                && (prev.unwrap().is_alphanumeric() || prev.unwrap() == '_')
            {
                continue;
            }

            for ch_str in token_matcher.match_str.chars() {
                match it.next() {
                    None => continue 'check_next_token,
                    Some(ch_text) => {
                        if ch_str != ch_text {
                            continue 'check_next_token;
                        }
                    }
                }
            }

            let after = it.next();
            if token_matcher.is_word
                && !after.is_none()
                && (after.unwrap().is_alphanumeric() || after.unwrap() == '_')
            {
                continue;
            }

            self.text.advance_by(token_matcher.match_str.len()).unwrap();
            return Ok(Some(token_matcher.token));
        }

        Ok(None)
    }

    fn get_next_token_identifier(&mut self) -> Result<Option<Token>, LexerError> {
        let mut it = self.text.clone();
        let first = it.next();

        if first.is_none() || (!first.unwrap().is_alphabetic() && first.unwrap() != '_') {
            return Ok(None);
        }

        let mut identifier = String::new();
        identifier.push(first.unwrap());

        while let Some(ch) = it.next() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }

            identifier.push(ch);
        }

        self.text.advance_by(identifier.len()).unwrap();
        Ok(Some(Token::Identifier(identifier)))
    }

    fn get_next_token_string_literal(&mut self) -> Result<Option<Token>, LexerError> {
        let mut it = self.text.clone();
        let first = it.next();

        if first.is_none() || first.unwrap() != '"' {
            return Ok(None);
        }

        let mut str = String::new();
        let mut escape = false;
        let mut num_chars = 0;

        while let Some(ch) = it.next() {
            if escape {
                if ch == '\\' {
                    str.push('\\')
                } else if ch == '\'' {
                    str.push('\'');
                } else if ch == '"' {
                    str.push('"');
                } else if ch == 'n' {
                    str.push('\n');
                } else if ch == 'r' {
                    str.push('\r');
                } else if ch == 't' {
                    str.push('\t');
                } else if ch == '0' {
                    str.push('\0');
                } else {
                    return Err(LexerError::InvalidEscapeSequenceInStringLiteral);
                }

                escape = false;
                num_chars += 1;
                continue;
            }

            if ch == '\\' {
                escape = true;
                num_chars += 1;
                continue;
            }

            if ch == '"' {
                self.text.advance_by(num_chars + 2).unwrap();
                return Ok(Some(Token::StringLiteral(str)));
            }

            str.push(ch);
            num_chars += 1;
        }

        Err(LexerError::UnterminatedStringLiteral)
    }

    fn get_next_token_integer_literal(&mut self) -> Result<Option<Token>, LexerError> {
        let mut it_radix = self.text.clone();
        let radix_first = it_radix.next();
        let radix_second = it_radix.next();
        let mut radix = 10;
        let mut it = self.text.clone();
        let mut num_chars = 0;

        if radix_first.is_some() && radix_first.unwrap() == '0' && radix_second.is_some() {
            if radix_second.unwrap() == 'x' {
                radix = 16;
                num_chars += 2;
                it = it_radix;
            } else if radix_second.unwrap() == 'b' {
                radix = 2;
                num_chars += 2;
                it = it_radix;
            } else if radix_second.unwrap() == 'o' {
                radix = 8;
                num_chars += 2;
                it = it_radix;
            }
        }

        let mut num_str = String::new();

        while let Some(ch) = it.next() {
            if !ch.is_digit(radix) {
                if ch.is_alphanumeric() || ch == '_' {
                    return Ok(None);
                }

                break;
            }

            num_str.push(ch);
            num_chars += 1;
        }

        if num_str.is_empty() {
            return Ok(None);
        }

        match u64::from_str_radix(&num_str, radix) {
            Err(_err) => Err(LexerError::TooLargeIntegerLiteral),
            Ok(n) => {
                self.text.advance_by(num_chars).unwrap();
                Ok(Some(Token::IntegerLiteral(n)))
            }
        }
    }

    fn get_next_token_char_literal(&mut self) -> Result<Option<Token>, LexerError> {
        let mut it = self.text.clone();
        let mut num_chars = 3;

        let first = it.next();
        if first.is_none() || first.unwrap() != '\'' {
            return Ok(None);
        }

        let mut ch = it.next();
        if ch.is_none() {
            return Err(LexerError::UnterminatedCharLiteral);
        }

        if ch.unwrap() == '\'' {
            return Err(LexerError::EmptyCharLiteral);
        }

        if ch.unwrap() == '\\' {
            num_chars += 1;

            let escape_char = it.next();
            if escape_char.is_none() {
                return Err(LexerError::UnterminatedCharLiteral);
            }

            if escape_char.unwrap() == '\\' {
                ch = Some('\\');
            } else if escape_char.unwrap() == '\'' {
                ch = Some('\'');
            } else if escape_char.unwrap() == '"' {
                ch = Some('"');
            } else if escape_char.unwrap() == 'n' {
                ch = Some('\n');
            } else if escape_char.unwrap() == 'r' {
                ch = Some('\r');
            } else if escape_char.unwrap() == 't' {
                ch = Some('\t');
            } else if escape_char.unwrap() == '0' {
                ch = Some('\0');
            } else {
                return Err(LexerError::InvalidEscapeSequenceInCharLiteral);
            }
        }

        let last = it.next();
        if last.is_none() || last.unwrap() != '\'' {
            return Err(LexerError::UnterminatedCharLiteral);
        }

        self.text.advance_by(num_chars).unwrap();
        Ok(Some(Token::IntegerLiteral(ch.unwrap() as u64)))
    }

    fn get_next_token(&mut self) -> Result<Option<Token>, LexerError> {
        if let Some(t) = self.get_next_token_simple()? {
            return Ok(Some(Token::SimpleToken(t)));
        }

        if let Some(t) = self.get_next_token_identifier()? {
            return Ok(Some(t));
        }

        if let Some(t) = self.get_next_token_string_literal()? {
            return Ok(Some(t));
        }

        if let Some(t) = self.get_next_token_integer_literal()? {
            return Ok(Some(t));
        }

        if let Some(t) = self.get_next_token_char_literal()? {
            return Ok(Some(t));
        }

        Ok(None)
    }
}

#[derive(Debug)]
pub enum LexerError {
    InvalidToken,
    InvalidEscapeSequenceInStringLiteral,
    InvalidEscapeSequenceInCharLiteral,
    EmptyCharLiteral,
    UnterminatedCharLiteral,
    UnterminatedStringLiteral,
    TooLargeIntegerLiteral,
}

#[cfg(test)]
mod tests {
    use super::{Lexer, SimpleToken, Token};

    #[test]
    fn test_get_tokens_empty() {
        let mut l = Lexer::new("");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_get_tokens_one() {
        let mut l = Lexer::new("+");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(tokens, vec![Token::SimpleToken(SimpleToken::Addition)]);
    }

    #[test]
    fn test_get_tokens_multiple() {
        let mut l = Lexer::new("++---");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::SimpleToken(SimpleToken::Addition),
                Token::SimpleToken(SimpleToken::Addition),
                Token::SimpleToken(SimpleToken::Subtraction),
                Token::SimpleToken(SimpleToken::Subtraction),
                Token::SimpleToken(SimpleToken::Subtraction),
            ]
        );
    }

    #[test]
    fn test_get_tokens_width() {
        let mut l = Lexer::new("***");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::SimpleToken(SimpleToken::Exponent),
                Token::SimpleToken(SimpleToken::Multiplication),
            ]
        );
    }

    #[test]
    fn test_get_tokens_identifier() {
        let mut l = Lexer::new("let mut abacus");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::SimpleToken(SimpleToken::Let),
                Token::SimpleToken(SimpleToken::Mut),
                Token::Identifier("abacus".to_string()),
            ]
        );
    }

    #[test]
    fn test_get_tokens_simple_word() {
        let mut l = Lexer::new("letmut abacus");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("letmut".to_string()),
                Token::Identifier("abacus".to_string()),
            ]
        );
    }

    #[test]
    fn test_get_tokens_whitespace() {
        let mut l = Lexer::new("+ - \t  * \n\n +   ");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::SimpleToken(SimpleToken::Addition),
                Token::SimpleToken(SimpleToken::Subtraction),
                Token::SimpleToken(SimpleToken::Multiplication),
                Token::SimpleToken(SimpleToken::Addition),
            ]
        );
    }

    #[test]
    fn test_get_tokens_declaration_with_string_literal() {
        let mut l: Lexer = Lexer::new("let mut a = \"abacus\";");
        let tokens = l.get_tokens().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::SimpleToken(SimpleToken::Let),
                Token::SimpleToken(SimpleToken::Mut),
                Token::Identifier("a".to_string()),
                Token::SimpleToken(SimpleToken::Assignment),
                Token::StringLiteral("abacus".to_string()),
                Token::SimpleToken(SimpleToken::Semicolon)
            ]
        );
    }

    #[test]
    fn test_get_tokens_declaration_with_empty_string_literal() {
        let mut l: Lexer = Lexer::new("let mut a = \"\";");
        let tokens = l.get_tokens().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::SimpleToken(SimpleToken::Let),
                Token::SimpleToken(SimpleToken::Mut),
                Token::Identifier("a".to_string()),
                Token::SimpleToken(SimpleToken::Assignment),
                Token::StringLiteral("".to_string()),
                Token::SimpleToken(SimpleToken::Semicolon)
            ]
        );
    }

    #[test]
    fn test_get_tokens_empty_string_literal() {
        let mut l: Lexer = Lexer::new("\"\"");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral("".to_string()),]);
    }

    #[test]
    fn test_get_tokens_integer_literals() {
        let mut l: Lexer = Lexer::new("12334759837459 123");
        let tokens = l.get_tokens().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::IntegerLiteral(12334759837459),
                Token::IntegerLiteral(123),
            ]
        );
    }

    #[test]
    fn test_get_tokens_integer_literals_base() {
        let mut l: Lexer = Lexer::new("0xFE 0b011 123 0o223");
        let tokens = l.get_tokens().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::IntegerLiteral(0xFE),
                Token::IntegerLiteral(0b011),
                Token::IntegerLiteral(123),
                Token::IntegerLiteral(0o223),
            ]
        );
    }

    #[test]
    fn test_get_tokens_string_literal_escape_sequence() {
        let mut tokens = Lexer::new("\"\\\"\"").get_tokens().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral("\"".to_string())]);
        tokens = Lexer::new("\"\\n\"").get_tokens().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral("\n".to_string())]);
    }

    #[test]
    fn test_get_tokens_char_literal() {
        let tokens = Lexer::new("'A' '\\\\'").get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::IntegerLiteral('A' as u64),
                Token::IntegerLiteral('\\' as u64)
            ]
        );
    }

    #[test]
    fn test_consume_whitespace() {
        let mut l = Lexer::new("   +  \n\t   -+   ");

        l.consume_whitespace();
        assert_eq!(l.text.next().unwrap(), '+');
        l.consume_whitespace();
        assert_eq!(l.text.next().unwrap(), '-');
        l.consume_whitespace();
        assert_eq!(l.text.next().unwrap(), '+');
        l.consume_whitespace();

        assert!(l.text.next().is_none());
    }
}
