use std::str::Chars;

pub struct Lexer<'a> {
    text: Chars<'a>,
}

impl Lexer<'_> {
    pub fn new(text: Chars<'_>) -> Lexer {
        Lexer { text }
    }

    pub fn get_tokens(&self) -> Result<Vec<Token>, LexerError> {
        let tokens: Vec<Token> = vec![];
        Ok(tokens)
    }
}

#[derive(Debug)]
pub enum LexerError {}

#[derive(Debug)]
pub enum Token {}
