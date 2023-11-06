use std::fmt::Binary;

use crate::{
    lexer::{SimpleToken, Token},
    parser::Parser,
    syntax_error::SyntaxError,
};

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Literal(Literal),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum BinaryOperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct BinaryOperation {
    operation_type: BinaryOperationType,
    left_expression: Box<Expression>,
    right_expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum UnaryOperationType {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub struct UnaryOperation {
    operation_type: UnaryOperationType,
    expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    StringLiteral(String),
    IntegerLiteral(u64),
}

impl Parser<'_> {
    pub fn get_next_expression(&mut self) -> Result<Option<Expression>, SyntaxError> {
        match self.get_next_primary()? {
            Some(primary) => self.get_next_expression_1(primary, 0),
            None => Ok(None),
        }
    }

    pub fn get_next_expression_1(
        &mut self,
        mut lhs: Expression,
        min_precedence: u32,
    ) -> Result<Option<Expression>, SyntaxError> {
        Ok(None)
    }

    fn get_next_primary(&mut self) -> Result<Option<Expression>, SyntaxError> {
        if let Some(expression) = self.get_next_expression_parens()? {
            return Ok(Some(expression));
        }

        if let Some(expression) = self.get_next_unit()? {
            return Ok(Some(expression));
        }

        if let Some(unary_operation) = self.get_next_unary_operation()? {
            return Ok(Some(Expression::UnaryOperation(unary_operation)));
        }

        Ok(None)
    }

    fn get_next_expression_parens(&mut self) -> Result<Option<Expression>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let next_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match next_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::LParen => {}
                _ => {
                    self.lexer = old_lexer;
                    return Ok(None);
                }
            },
            _ => {
                self.lexer = old_lexer;
                return Ok(None);
            }
        }

        let expression = match self.get_next_expression()? {
            None => return Err(SyntaxError::NoExpressionAfterLParen),
            Some(expression) => expression,
        };

        let last_token = match self.lexer.get_next_token()? {
            None => {
                return Err(SyntaxError::UnmatchedParen);
            }
            Some(token) => token,
        };

        match last_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::RParen => {}
                _ => {
                    return Err(SyntaxError::UnmatchedParen);
                }
            },
            _ => {
                return Err(SyntaxError::UnmatchedParen);
            }
        }

        Ok(Some(expression))
    }

    fn get_next_unit(&mut self) -> Result<Option<Expression>, SyntaxError> {
        let old_lexer = self.lexer.clone();
        let token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        let expression = match token {
            Token::Identifier(identifier) => Expression::Identifier(identifier),
            Token::IntegerLiteral(literal) => Expression::Literal(Literal::IntegerLiteral(literal)),
            Token::StringLiteral(literal) => Expression::Literal(Literal::StringLiteral(literal)),
            _ => {
                self.lexer = old_lexer;
                return Ok(None);
            }
        };

        Ok(Some(expression))
    }

    fn get_next_unary_operation(&mut self) -> Result<Option<UnaryOperation>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        let operation_type = match first_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::Addition => UnaryOperationType::Plus,
                SimpleToken::Subtraction => UnaryOperationType::Minus,
                _ => {
                    self.lexer = old_lexer;
                    return Ok(None);
                }
            },
            _ => {
                self.lexer = old_lexer;
                return Ok(None);
            }
        };

        let primary = match self.get_next_primary()? {
            None => return Err(SyntaxError::NoExpressionAfterUnaryOperator),
            Some(token) => token,
        };

        Ok(Some(UnaryOperation {
            operation_type,
            expression: Box::new(primary),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn test_get_next_primary() {
        let mut p = Parser::new("-((a+1) + (a+2))");
        let e = p.get_next_primary().unwrap().unwrap();
        println!("{:?}", e);
    }

    #[test]
    fn test_get_next_expression() {
        let mut p = Parser::new("1+1*2");
        let e = p.get_next_expression().unwrap().unwrap();
        println!("{:?}", e);
    }
}
