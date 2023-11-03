use crate::current_iterator::CurrentIterator;
use std::str::Chars;

#[derive(Debug, Eq, PartialEq)]
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

        while let Some(ch) = it.next() {
            if ch == '"' {
                self.text.advance_by(str.len() + 2).unwrap();
                return Ok(Some(Token::StringLiteral(str)));
            }

            str.push(ch);
        }

        Err(LexerError::UnterminatedStringLiteral)
    }

    fn get_next_token_integer_literal(&mut self) -> Result<Option<Token>, LexerError> {
        let mut it = self.text.clone();
        let mut num_str = String::new();

        while let Some(ch) = it.next() {
            if !ch.is_digit(10) {
                if ch.is_alphanumeric() || ch == '_' {
                    return Ok(None);
                }

                break;
            }

            num_str.push(ch);
        }

        if num_str.is_empty() {
            return Ok(None);
        }

        match num_str.parse::<u64>() {
            Err(_err) => Err(LexerError::TooLargeIntegerLiteral),
            Ok(n) => {
                self.text.advance_by(num_str.len()).unwrap();
                Ok(Some(Token::IntegerLiteral(n)))
            }
        }
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

        Ok(None)
    }
}

#[derive(Debug)]
pub enum LexerError {
    InvalidToken,
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
