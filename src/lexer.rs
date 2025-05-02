use crate::current_iterator::CurrentIterator;
use crate::syntax_error::SyntaxError;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    SimpleToken(SimpleToken),
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(u64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimpleToken {
    // Keywords
    Let,
    Mut,
    If,
    Else,
    Break,
    Continue,
    Loop,
    While,
    // Unary Operators
    LogicalNot,
    BitwiseNot,
    // Binary Operators
    Exponentiate,
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,
    LeftShift,
    RightShift,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    // Assignment Operators
    ExponentiationAssignment,
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment,
    ModulusAssignment,
    BitwiseAndAssignment,
    BitwiseOrAssignment,
    BitwiseXorAssignment,
    LeftShiftAssignment,
    RightShiftAssignment,
    Assignment,
    // Other
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
}

struct SimpleTokenMatcher {
    token: SimpleToken,
    match_str: &'static str,
    is_word: bool,
}

#[derive(Clone)]
pub struct Lexer<'a> {
    text: CurrentIterator<Chars<'a>>,
}

const MATCH_TOKENS: [SimpleTokenMatcher; 47] = [
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
        token: SimpleToken::If,
        match_str: "if",
        is_word: true,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Else,
        match_str: "else",
        is_word: true,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Break,
        match_str: "break",
        is_word: true,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Continue,
        match_str: "continue",
        is_word: true,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Loop,
        match_str: "loop",
        is_word: true,
    },
    SimpleTokenMatcher {
        token: SimpleToken::While,
        match_str: "while",
        is_word: true,
    },
    SimpleTokenMatcher {
        token: SimpleToken::ExponentiationAssignment,
        match_str: "**=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::LeftShiftAssignment,
        match_str: "<<=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::RightShiftAssignment,
        match_str: ">>=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::AdditionAssignment,
        match_str: "+=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::SubtractionAssignment,
        match_str: "-=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::MultiplicationAssignment,
        match_str: "*=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::DivisionAssignment,
        match_str: "/=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::ModulusAssignment,
        match_str: "%=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::BitwiseAndAssignment,
        match_str: "&=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::BitwiseOrAssignment,
        match_str: "|=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::BitwiseXorAssignment,
        match_str: "^=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Exponentiate,
        match_str: "**",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::LeftShift,
        match_str: "<<",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::RightShift,
        match_str: ">>",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::GreaterEqual,
        match_str: ">=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::LessEqual,
        match_str: "<=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Equal,
        match_str: "==",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::NotEqual,
        match_str: "!=",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::LogicalAnd,
        match_str: "&&",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::LogicalOr,
        match_str: "||",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Multiply,
        match_str: "*",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::BitwiseNot,
        match_str: "~",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::LogicalNot,
        match_str: "!",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Divide,
        match_str: "/",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Modulus,
        match_str: "%",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Add,
        match_str: "+",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Subtract,
        match_str: "-",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Greater,
        match_str: ">",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::Less,
        match_str: "<",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::BitwiseAnd,
        match_str: "&",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::BitwiseXor,
        match_str: "^",
        is_word: false,
    },
    SimpleTokenMatcher {
        token: SimpleToken::BitwiseOr,
        match_str: "|",
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
        token: SimpleToken::Comma,
        match_str: ",",
        is_word: false,
    },
];

impl Lexer<'_> {
    pub fn new<'a>(input_data: &'a str) -> Lexer<'a> {
        Lexer {
            text: CurrentIterator::new(input_data.chars()),
        }
    }

    #[allow(dead_code)]
    pub fn get_tokens(&mut self) -> Result<Vec<Token>, SyntaxError> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = self.get_next_token()? {
            tokens.push(token);
        }

        self.consume_whitespace();

        // If there is more text, an invalid token was encountered.
        let mut it = self.text.clone();
        if it.next().is_some() {
            return Err(SyntaxError::InvalidToken);
        }

        Ok(tokens)
    }

    // Returns true iff EOF.
    fn consume_whitespace(&mut self) -> bool {
        loop {
            let mut it = self.text.clone();
            match it.next() {
                None => return true,
                Some(ch) => {
                    if !ch.is_whitespace() {
                        return false;
                    }
                    self.text.next();
                }
            }
        }
    }

    fn get_next_token_simple(&mut self) -> Result<Option<SimpleToken>, SyntaxError> {
        'check_next_token: for token_matcher in MATCH_TOKENS {
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

            for _ in 0..token_matcher.match_str.len() {
                self.text.next();
            }
            return Ok(Some(token_matcher.token));
        }

        Ok(None)
    }

    fn get_next_token_identifier(&mut self) -> Result<Option<Token>, SyntaxError> {
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

        for _ in 0..identifier.len() {
            self.text.next();
        }
        Ok(Some(Token::Identifier(identifier)))
    }

    fn get_next_token_string_literal(&mut self) -> Result<Option<Token>, SyntaxError> {
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
                    return Err(SyntaxError::InvalidEscapeSequenceInStringLiteral);
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
                for _ in 0..num_chars + 2 {
                    self.text.next();
                }
                return Ok(Some(Token::StringLiteral(str)));
            }

            str.push(ch);
            num_chars += 1;
        }

        Err(SyntaxError::UnterminatedStringLiteral)
    }

    fn get_next_token_integer_literal(&mut self) -> Result<Option<Token>, SyntaxError> {
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
            Err(_err) => Err(SyntaxError::TooLargeIntegerLiteral),
            Ok(n) => {
                for _ in 0..num_chars {
                    self.text.next();
                }
                Ok(Some(Token::IntegerLiteral(n)))
            }
        }
    }

    fn get_next_token_char_literal(&mut self) -> Result<Option<Token>, SyntaxError> {
        let mut it = self.text.clone();
        let mut num_chars = 3;

        let first = it.next();
        if first.is_none() || first.unwrap() != '\'' {
            return Ok(None);
        }

        let mut ch = it.next();
        if ch.is_none() {
            return Err(SyntaxError::UnterminatedCharLiteral);
        }

        if ch.unwrap() == '\'' {
            return Err(SyntaxError::EmptyCharLiteral);
        }

        if ch.unwrap() == '\\' {
            num_chars += 1;

            let escape_char = it.next();
            if escape_char.is_none() {
                return Err(SyntaxError::UnterminatedCharLiteral);
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
                return Err(SyntaxError::InvalidEscapeSequenceInCharLiteral);
            }
        }

        let last = it.next();
        if last.is_none() || last.unwrap() != '\'' {
            return Err(SyntaxError::UnterminatedCharLiteral);
        }

        for _ in 0..num_chars {
            self.text.next();
        }
        Ok(Some(Token::IntegerLiteral(ch.unwrap() as u64)))
    }

    pub fn get_next_token(&mut self) -> Result<Option<Token>, SyntaxError> {
        if self.consume_whitespace() {
            return Ok(None);
        };

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
        assert_eq!(tokens, vec![Token::SimpleToken(SimpleToken::Add)]);
    }

    #[test]
    fn test_get_tokens_multiple() {
        let mut l = Lexer::new("++---");
        let tokens = l.get_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::SimpleToken(SimpleToken::Add),
                Token::SimpleToken(SimpleToken::Add),
                Token::SimpleToken(SimpleToken::Subtract),
                Token::SimpleToken(SimpleToken::Subtract),
                Token::SimpleToken(SimpleToken::Subtract),
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
                Token::SimpleToken(SimpleToken::Exponentiate),
                Token::SimpleToken(SimpleToken::Multiply),
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
                Token::SimpleToken(SimpleToken::Add),
                Token::SimpleToken(SimpleToken::Subtract),
                Token::SimpleToken(SimpleToken::Multiply),
                Token::SimpleToken(SimpleToken::Add),
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
