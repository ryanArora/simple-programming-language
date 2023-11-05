use crate::{
    lexer::{SimpleToken, Token},
    parser::Parser,
    syntax_error::SyntaxError,
};

#[derive(Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Literal(Literal),
    Identifier(String),
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
        let old_lexer = self.lexer.clone();

        let mut expression = match self.get_next_product()? {
            Some(token) => token,
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
        };

        loop {
            let old_lexer = self.lexer.clone();

            let next_token = match self.lexer.get_next_token()? {
                Some(token) => token,
                None => {
                    self.lexer = old_lexer;
                    break;
                }
            };

            let operation_type: BinaryOperationType = match next_token {
                Token::SimpleToken(simple_token) => match simple_token {
                    SimpleToken::Addition => BinaryOperationType::Add,
                    SimpleToken::Subtraction => BinaryOperationType::Subtract,
                    _ => {
                        self.lexer = old_lexer;
                        break;
                    }
                },
                _ => {
                    self.lexer = old_lexer;
                    break;
                }
            };

            let next_product = match self.get_next_product()? {
                Some(token) => token,
                None => {
                    self.lexer = old_lexer;
                    break;
                }
            };

            expression = Expression::BinaryOperation(BinaryOperation {
                operation_type,
                left_expression: Box::new(expression),
                right_expression: Box::new(next_product),
            });
        }

        Ok(Some(expression))
    }

    fn get_next_product(&mut self) -> Result<Option<Expression>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let next_identifier = match self.lexer.get_next_token()? {
            Some(next_token) => match next_token {
                Token::Identifier(identifier) => identifier,
                _ => {
                    self.lexer = old_lexer;
                    return Ok(None);
                }
            },
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
        };

        Ok(Some(Expression::Identifier(next_identifier)))
    }
}
