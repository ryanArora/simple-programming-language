use crate::ast::block::Block;
use crate::ast::statement::Statement;
use crate::lexer::Lexer;
use crate::syntax_error::SyntaxError;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new<'a>(input_data: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(input_data),
        }
    }

    pub fn get_ast(&mut self) -> Result<Option<Block>, SyntaxError> {
        let mut statements: Vec<Statement> = vec![];
        while let Some(statement) = Parser::get_next_statement(self)? {
            statements.push(statement);
        }

        if self.lexer.get_next_token()?.is_some() {
            return Err(SyntaxError::InvalidToken);
        }

        let block = Block { statements };
        Ok(Some(block))
    }
}
