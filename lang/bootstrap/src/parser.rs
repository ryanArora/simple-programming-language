use crate::lexer;
use lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn get_ast(&self) -> Result<Ast, ParserError> {
        Ok(Ast {})
    }
}

#[derive(Debug)]
pub enum ParserError {}

pub struct Ast {}
