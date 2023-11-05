use crate::{lexer::Lexer, parser::Parser, syntax_error::SyntaxError};

#[derive(Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Literal(Literal),
}

#[derive(Debug)]
pub enum BinaryOperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct BinaryOperation {
    operation_type: BinaryOperationType,
    left_expression: Box<Expression>,
    right_expression: Box<Expression>,
}

#[derive(Debug)]
pub enum UnaryOperationType {
    Plus,
    Minus,
}

#[derive(Debug)]
pub struct UnaryOperation {
    operation_type: UnaryOperationType,
    expression: Box<Expression>,
}

#[derive(Debug)]
pub enum Literal {
    StringLiteral(String),
    IntegerLiteral(u64),
}

impl Parser<'_> {
    pub fn get_next_expression(&mut self) -> Result<Option<Expression>, SyntaxError> {
        println!("get_next_expression");
        Ok(None)
    }
}
