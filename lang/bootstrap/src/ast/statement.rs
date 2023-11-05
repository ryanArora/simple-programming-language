use crate::{ast::expression::Expression, lexer::Lexer, parser::Parser, syntax_error::SyntaxError};

#[derive(Debug)]
pub enum Statement {
    Assignment(AssignmentStatement),
    Expression(Expression),
}

#[derive(Debug)]
pub struct AssignmentStatement {
    identifier: String,
    expression: Expression,
}

impl Parser<'_> {
    pub fn get_next_statement(lexer: &Lexer) -> Result<Option<Statement>, SyntaxError> {
        Ok(None)
    }
}
