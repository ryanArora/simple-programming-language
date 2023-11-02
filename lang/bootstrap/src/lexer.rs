use std::str::Chars;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Token {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
}

#[derive(Clone, Copy)]
struct TokenMatcher {
    token: Token,
    match_str: &'static str,
    is_word: bool,
}

pub struct Lexer<'a> {
    text: Chars<'a>,
    match_tokens: Vec<TokenMatcher>,
}

impl Lexer<'_> {
    pub fn new<'a>(input_data: &'a str) -> Lexer<'a> {
        Lexer {
            text: input_data.chars(),
            match_tokens: vec![
                TokenMatcher {
                    token: Token::Exponent,
                    match_str: "**",
                    is_word: false,
                },
                TokenMatcher {
                    token: Token::Addition,
                    match_str: "+",
                    is_word: false,
                },
                TokenMatcher {
                    token: Token::Subtraction,
                    match_str: "-",
                    is_word: false,
                },
                TokenMatcher {
                    token: Token::Multiplication,
                    match_str: "*",
                    is_word: false,
                },
                TokenMatcher {
                    token: Token::Division,
                    match_str: "/",
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

    fn get_next_token(&mut self) -> Result<Option<Token>, LexerError> {
        'check_next_token: for &token_matcher in &self.match_tokens {
            let mut it = self.text.clone();

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

            self.text.advance_by(token_matcher.match_str.len()).unwrap();
            return Ok(Some(token_matcher.token));
        }

        Ok(None)
    }
}

#[derive(Debug)]
pub enum LexerError {
    InvalidToken,
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

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
        assert_eq!(tokens, vec![Token::Addition]);
    }

    #[test]
    fn test_get_tokens_multiple() {
        let mut l = Lexer::new("++---");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Addition,
                Token::Addition,
                Token::Subtraction,
                Token::Subtraction,
                Token::Subtraction,
            ]
        );
    }

    #[test]
    fn test_get_tokens_width() {
        let mut l = Lexer::new("***");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(tokens, vec![Token::Exponent, Token::Multiplication]);
    }

    #[test]
    fn test_get_tokens_whitespace() {
        let mut l = Lexer::new("+ - \t  * \n\n +   ");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Addition,
                Token::Subtraction,
                Token::Multiplication,
                Token::Addition
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
