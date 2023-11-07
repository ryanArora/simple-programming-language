use crate::{
    lexer::{SimpleToken, Token},
    parser::Parser,
    syntax_error::SyntaxError,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Literal(Literal),
    Identifier(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperationType {
    Add = SimpleToken::Add as isize,
    Subtract = SimpleToken::Subtract as isize,
    Multiply = SimpleToken::Multiply as isize,
    Divide = SimpleToken::Divide as isize,
    Modulus = SimpleToken::Modulus as isize,
    Exponentiate = SimpleToken::Exponentiate as isize,
    Equal = SimpleToken::Equal as isize,
    NotEqual = SimpleToken::NotEqual as isize,
    GreaterEqual = SimpleToken::GreaterEqual as isize,
    LessEqual = SimpleToken::LessEqual as isize,
    Greater = SimpleToken::Greater as isize,
    Less = SimpleToken::Less as isize,
    LogicalAnd = SimpleToken::LogicalAnd as isize,
    LogicalOr = SimpleToken::LogicalOr as isize,
    BitwiseAnd = SimpleToken::BitwiseAnd as isize,
    BitwiseOr = SimpleToken::BitwiseOr as isize,
    BitwiseXor = SimpleToken::BitwiseXor as isize,
    LeftShift = SimpleToken::LeftShift as isize,
    RightShift = SimpleToken::RightShift as isize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    operation_type: BinaryOperationType,
    left_expression: Box<Expression>,
    right_expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperationType {
    LogicalNot = SimpleToken::LogicalAnd as isize,
    BitwiseNot = SimpleToken::LogicalNot as isize,
    Plus = SimpleToken::Add as isize,
    Minus = SimpleToken::Subtract as isize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    operation_type: UnaryOperationType,
    expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    StringLiteral(String),
    IntegerLiteral(u64),
}

fn token_to_unary_operation_type(token: Token) -> Option<UnaryOperationType> {
    Some(match token {
        Token::SimpleToken(simple_token) => match simple_token {
            SimpleToken::LogicalNot => UnaryOperationType::LogicalNot,
            SimpleToken::BitwiseNot => UnaryOperationType::BitwiseNot,
            SimpleToken::Add => UnaryOperationType::Plus,
            SimpleToken::Subtract => UnaryOperationType::Minus,
            _ => return None,
        },
        _ => return None,
    })
}

fn token_to_binary_operation_type(token: Token) -> Option<BinaryOperationType> {
    Some(match token {
        Token::SimpleToken(simple_token) => match simple_token {
            SimpleToken::Add => BinaryOperationType::Add,
            SimpleToken::Subtract => BinaryOperationType::Subtract,
            SimpleToken::Multiply => BinaryOperationType::Multiply,
            SimpleToken::Divide => BinaryOperationType::Divide,
            SimpleToken::Modulus => BinaryOperationType::Modulus,
            SimpleToken::Exponentiate => BinaryOperationType::Exponentiate,
            SimpleToken::Equal => BinaryOperationType::Equal,
            SimpleToken::NotEqual => BinaryOperationType::NotEqual,
            SimpleToken::GreaterEqual => BinaryOperationType::GreaterEqual,
            SimpleToken::LessEqual => BinaryOperationType::LessEqual,
            SimpleToken::Greater => BinaryOperationType::Greater,
            SimpleToken::Less => BinaryOperationType::Less,
            SimpleToken::LogicalAnd => BinaryOperationType::LogicalAnd,
            SimpleToken::LogicalOr => BinaryOperationType::LogicalOr,
            SimpleToken::BitwiseAnd => BinaryOperationType::BitwiseAnd,
            SimpleToken::BitwiseOr => BinaryOperationType::BitwiseOr,
            SimpleToken::BitwiseXor => BinaryOperationType::BitwiseXor,
            SimpleToken::LeftShift => BinaryOperationType::LeftShift,
            SimpleToken::RightShift => BinaryOperationType::RightShift,
            _ => return None,
        },
        _ => return None,
    })
}

fn get_operator_precedence(op: BinaryOperationType) -> u32 {
    match op {
        BinaryOperationType::Exponentiate => 10,
        BinaryOperationType::Multiply => 9,
        BinaryOperationType::Divide => 9,
        BinaryOperationType::Modulus => 9,
        BinaryOperationType::Add => 8,
        BinaryOperationType::Subtract => 8,
        BinaryOperationType::LeftShift => 7,
        BinaryOperationType::RightShift => 7,
        BinaryOperationType::Greater => 6,
        BinaryOperationType::Less => 6,
        BinaryOperationType::GreaterEqual => 6,
        BinaryOperationType::LessEqual => 6,
        BinaryOperationType::Equal => 5,
        BinaryOperationType::NotEqual => 5,
        BinaryOperationType::BitwiseAnd => 4,
        BinaryOperationType::BitwiseXor => 3,
        BinaryOperationType::BitwiseOr => 2,
        BinaryOperationType::LogicalAnd => 1,
        BinaryOperationType::LogicalOr => 0,
    }
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
        let mut old_lexer;
        loop {
            old_lexer = self.lexer.clone();

            let lookahead_token = match self.lexer.get_next_token()? {
                None => {
                    self.lexer = old_lexer;
                    break;
                }
                Some(token) => token,
            };

            let op = match token_to_binary_operation_type(lookahead_token) {
                Some(op) => op,
                None => {
                    self.lexer = old_lexer;
                    break;
                }
            };

            let p = get_operator_precedence(op);

            if !(p >= min_precedence) {
                self.lexer = old_lexer;
                break;
            }

            let mut rhs = match self.get_next_primary()? {
                None => return Err(SyntaxError::NoExpressionAfterBinaryOperator),
                Some(primary) => primary,
            };

            loop {
                old_lexer = self.lexer.clone();

                let lookahead_token = match self.lexer.get_next_token()? {
                    None => {
                        self.lexer = old_lexer;
                        break;
                    }
                    Some(token) => token,
                };

                let op2 = match token_to_binary_operation_type(lookahead_token) {
                    Some(op) => op,
                    None => {
                        self.lexer = old_lexer;
                        break;
                    }
                };

                let p2 = get_operator_precedence(op2);

                if !(p2 > p) {
                    self.lexer = old_lexer;
                    break;
                }

                self.lexer = old_lexer;

                rhs = match self.get_next_expression_1(rhs, p + 1)? {
                    None => return Err(SyntaxError::InvalidToken),
                    Some(expression) => expression,
                };
            }

            lhs = Expression::BinaryOperation(BinaryOperation {
                operation_type: op,
                left_expression: Box::new(lhs),
                right_expression: Box::new(rhs),
            });
        }

        Ok(Some(lhs))
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

        let operation_type = match token_to_unary_operation_type(first_token) {
            Some(op) => op,
            None => {
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

    use super::{BinaryOperation, BinaryOperationType, Expression};

    #[test]
    fn test_get_next_expression() {
        let mut p = Parser::new("a + b * c + (-d)");
        let e = p.get_next_expression().unwrap().unwrap();

        let a = Expression::Identifier("a".to_string());
        let b = Expression::Identifier("b".to_string());
        let c = Expression::Identifier("c".to_string());
        let d = Expression::Identifier("d".to_string());
        let minus_d = Expression::UnaryOperation(super::UnaryOperation {
            operation_type: super::UnaryOperationType::Minus,
            expression: Box::new(d),
        });
        let b_times_c = Expression::BinaryOperation(BinaryOperation {
            operation_type: BinaryOperationType::Multiply,
            left_expression: Box::new(b),
            right_expression: Box::new(c),
        });
        let a_plus_b_times_c = Expression::BinaryOperation(BinaryOperation {
            operation_type: BinaryOperationType::Add,
            left_expression: Box::new(a),
            right_expression: Box::new(b_times_c),
        });

        let a_plus_b_times_c_plus_minus_d = Expression::BinaryOperation(BinaryOperation {
            operation_type: BinaryOperationType::Add,
            left_expression: Box::new(a_plus_b_times_c),
            right_expression: Box::new(minus_d),
        });

        assert_eq!(e, a_plus_b_times_c_plus_minus_d);
    }
}
