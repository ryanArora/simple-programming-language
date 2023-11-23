use crate::{
    ast::block::Block,
    ast::expression::Expression,
    lexer::{SimpleToken, Token},
    parser::Parser,
    syntax_error::SyntaxError,
};

use super::expression::{BinaryOperation, BinaryOperationType};

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    Assignment(AssignmentStatement),
    IfStatement(IfStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    LoopStatement(LoopStatement),
    WhileStatement(WhileStatement),
    Expression(Expression),
    EmptyStatement,
}

#[derive(Debug)]
pub struct LetStatement {
    pub identifier: String,
    pub mutable: bool,
    pub expression: Option<Expression>,
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    pub _if: ConditionWithBlock,
    pub else_if: Vec<ConditionWithBlock>,
    pub _else: Option<Block>,
}

#[derive(Debug)]
pub struct ConditionWithBlock {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Debug)]
pub struct BreakStatement;
#[derive(Debug)]
pub struct ContinueStatement;

#[derive(Debug)]
pub struct LoopStatement {
    pub block: Block,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub block: Block,
}

impl Parser<'_> {
    pub fn consume_semicolon(&mut self) -> Result<Option<()>, SyntaxError> {
        let old_lexer = self.lexer.clone();
        let token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::Semicolon => return Ok(Some(())),
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
    }

    pub fn get_next_statement(&mut self) -> Result<Option<Statement>, SyntaxError> {
        let mut next_statement: Option<Statement> = None;

        if let Some(statement) = self.get_next_let_statement()? {
            next_statement = Some(Statement::LetStatement(statement));
        } else if let Some(statement) = self.get_next_assignment_statement()? {
            next_statement = Some(Statement::Assignment(statement));
        } else if let Some(statement) = self.get_next_if_statement()? {
            next_statement = Some(Statement::IfStatement(statement));
        } else if let Some(_) = self.get_next_break_statement()? {
            next_statement = Some(Statement::BreakStatement(BreakStatement));
        } else if let Some(_) = self.get_next_continue_statement()? {
            next_statement = Some(Statement::ContinueStatement(ContinueStatement));
        } else if let Some(statement) = self.get_next_loop_statement()? {
            next_statement = Some(Statement::LoopStatement(statement));
        } else if let Some(statement) = self.get_next_while_statement()? {
            next_statement = Some(Statement::WhileStatement(statement));
        } else if let Some(statement) = self.get_next_expression()? {
            next_statement = Some(Statement::Expression(statement));
        }

        match next_statement {
            None => match self.consume_semicolon()? {
                None => return Ok(None),
                Some(_) => return Ok(Some(Statement::EmptyStatement)),
            },
            Some(_) => match self.consume_semicolon()? {
                None => return Err(SyntaxError::StatementWithoutSemicolon),
                Some(_) => return Ok(next_statement),
            },
        }
    }

    fn get_next_let_statement(&mut self) -> Result<Option<LetStatement>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match first_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::Let => {}
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

        let old_lexer = self.lexer.clone();

        let mutable = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                false
            }
            Some(token) => match token {
                Token::SimpleToken(simple_token) => match simple_token {
                    SimpleToken::Mut => true,
                    _ => {
                        self.lexer = old_lexer;
                        false
                    }
                },
                _ => {
                    self.lexer = old_lexer;
                    false
                }
            },
        };

        let next_token = match self.lexer.get_next_token()? {
            None => return Err(SyntaxError::NoIdentifierInLetStatement),
            Some(token) => token,
        };

        let identifier = match next_token {
            Token::Identifier(identifier) => identifier,
            _ => return Err(SyntaxError::NoIdentifierInLetStatement),
        };

        let old_lexer = self.lexer.clone();

        let expression = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                None
            }
            Some(token) => match token {
                Token::SimpleToken(simple_token) => match simple_token {
                    SimpleToken::Assignment => match self.get_next_expression()? {
                        None => return Err(SyntaxError::NoExpressionInLetAssignmentStatement),
                        Some(expression) => Some(expression),
                    },
                    _ => {
                        self.lexer = old_lexer;
                        None
                    }
                },
                _ => {
                    self.lexer = old_lexer;
                    None
                }
            },
        };

        Ok(Some(LetStatement {
            identifier,
            mutable,
            expression,
        }))
    }

    fn get_next_assignment_statement(
        &mut self,
    ) -> Result<Option<AssignmentStatement>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        let identifier = match first_token {
            Token::Identifier(identifier) => identifier,
            _ => {
                self.lexer = old_lexer;
                return Ok(None);
            }
        };

        let second_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        let binary_operation_type = match second_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::ExponentiationAssignment => Some(BinaryOperationType::Exponentiate),
                SimpleToken::AdditionAssignment => Some(BinaryOperationType::Add),
                SimpleToken::SubtractionAssignment => Some(BinaryOperationType::Subtract),
                SimpleToken::MultiplicationAssignment => Some(BinaryOperationType::Multiply),
                SimpleToken::DivisionAssignment => Some(BinaryOperationType::Divide),
                SimpleToken::ModulusAssignment => Some(BinaryOperationType::Modulus),
                SimpleToken::BitwiseAndAssignment => Some(BinaryOperationType::BitwiseAnd),
                SimpleToken::BitwiseOrAssignment => Some(BinaryOperationType::BitwiseOr),
                SimpleToken::BitwiseXorAssignment => Some(BinaryOperationType::BitwiseXor),
                SimpleToken::LeftShiftAssignment => Some(BinaryOperationType::LeftShift),
                SimpleToken::RightShiftAssignment => Some(BinaryOperationType::RightShift),
                SimpleToken::Assignment => None,
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

        let expression = match self.get_next_expression()? {
            None => return Err(SyntaxError::NoExpressionInAssignmentStatement),
            Some(expression) => expression,
        };

        let assignment_statement = match binary_operation_type {
            None => AssignmentStatement {
                identifier,
                expression,
            },
            Some(operation_type) => {
                let ident = identifier.clone();
                AssignmentStatement {
                    identifier,
                    expression: Expression::BinaryOperation(BinaryOperation {
                        operation_type,
                        left_expression: Box::new(Expression::Identifier(ident)),
                        right_expression: Box::new(expression),
                    }),
                }
            }
        };

        Ok(Some(assignment_statement))
    }

    fn get_next_if_statement(&mut self) -> Result<Option<IfStatement>, SyntaxError> {
        let _if = match self.get_next_if_statement_1()? {
            None => return Ok(None),
            Some(_if) => _if,
        };
        let else_if = self.get_next_if_statement_2()?;
        let _else = self.get_next_if_statement_3()?;

        Ok(Some(IfStatement {
            _if,
            else_if,
            _else,
        }))
    }

    fn get_next_if_statement_1(&mut self) -> Result<Option<ConditionWithBlock>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match first_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::If => {}
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

        let if_condition = match self.get_next_expression()? {
            None => return Err(SyntaxError::NoConditionInIfStatement),
            Some(block) => block,
        };

        let if_block = match self.get_next_block()? {
            None => return Err(SyntaxError::NoBlockInIfStatement),
            Some(block) => block,
        };

        Ok(Some(ConditionWithBlock {
            condition: if_condition,
            block: if_block,
        }))
    }

    fn get_next_if_statement_2(&mut self) -> Result<Vec<ConditionWithBlock>, SyntaxError> {
        let mut _else_if: Vec<ConditionWithBlock> = vec![];

        loop {
            let old_lexer = self.lexer.clone();

            let first_token = match self.lexer.get_next_token()? {
                None => {
                    self.lexer = old_lexer;
                    break;
                }
                Some(token) => token,
            };

            match first_token {
                Token::SimpleToken(simple_token) => match simple_token {
                    SimpleToken::Else => {}
                    _ => {
                        self.lexer = old_lexer;
                        break;
                    }
                },
                _ => {
                    self.lexer = old_lexer;
                    break;
                }
            }

            let second_token = match self.lexer.get_next_token()? {
                None => {
                    self.lexer = old_lexer;
                    break;
                }
                Some(token) => token,
            };

            match second_token {
                Token::SimpleToken(simple_token) => match simple_token {
                    SimpleToken::If => {}
                    _ => {
                        self.lexer = old_lexer;
                        break;
                    }
                },
                _ => {
                    self.lexer = old_lexer;
                    break;
                }
            }

            let else_if_condition = match self.get_next_expression()? {
                None => return Err(SyntaxError::NoConditionInElseIfStatement),
                Some(block) => block,
            };

            let else_if_block = match self.get_next_block()? {
                None => return Err(SyntaxError::NoBlockInElseIfStatement),
                Some(block) => block,
            };

            _else_if.push(ConditionWithBlock {
                condition: else_if_condition,
                block: else_if_block,
            });
        }

        Ok(_else_if)
    }

    fn get_next_if_statement_3(&mut self) -> Result<Option<Block>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match first_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::Else => {}
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

        let else_block = match self.get_next_block()? {
            None => return Err(SyntaxError::NoBlockInElseStatement),
            Some(block) => block,
        };

        Ok(Some(else_block))
    }

    fn get_next_break_statement(&mut self) -> Result<Option<()>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match first_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::Break => return Ok(Some(())),
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
    }

    fn get_next_continue_statement(&mut self) -> Result<Option<()>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match first_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::Continue => return Ok(Some(())),
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
    }

    fn get_next_loop_statement(&mut self) -> Result<Option<LoopStatement>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match first_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::Loop => {}
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

        let block = match self.get_next_block()? {
            None => return Err(SyntaxError::NoBlockInLoopStatement),
            Some(block) => block,
        };

        Ok(Some(LoopStatement { block }))
    }

    fn get_next_while_statement(&mut self) -> Result<Option<WhileStatement>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        match first_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::While => {}
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

        let condition = match self.get_next_expression()? {
            None => return Err(SyntaxError::NoConditionInWhileStatement),
            Some(condition) => condition,
        };

        let block = match self.get_next_block()? {
            None => return Err(SyntaxError::NoBlockInWhileStatement),
            Some(block) => block,
        };

        Ok(Some(WhileStatement { condition, block }))
    }
}
