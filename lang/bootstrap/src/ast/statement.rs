use crate::{
    ast::block::Block,
    ast::expression::Expression,
    lexer::{SimpleToken, Token},
    parser::Parser,
    syntax_error::SyntaxError,
};

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    Assignment(AssignmentStatement),
    IfStatement(IfStatement),
    Expression(Expression),
    EmptyStatement,
}

#[derive(Debug)]
pub struct LetStatement {
    identifier: String,
    mutable: bool,
}

#[derive(Debug)]
pub struct AssignmentStatement {
    identifier: String,
    expression: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    _if: ConditionWithBlock,
    else_if: Vec<ConditionWithBlock>,
    _else: Option<Block>,
}

#[derive(Debug)]
pub struct ConditionWithBlock {
    condition: Expression,
    block: Block,
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
        if let Some(let_statement) = self.get_next_let_statement()? {
            next_statement = Some(Statement::LetStatement(let_statement));
        } else if let Some(assignment) = self.get_next_assignment_statement()? {
            next_statement = Some(Statement::Assignment(assignment));
        } else if let Some(if_statement) = self.get_next_if_statement()? {
            next_statement = Some(Statement::IfStatement(if_statement));
        } else if let Some(expression) = self.get_next_expression()? {
            next_statement = Some(Statement::Expression(expression));
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

        let second_token = match self.lexer.get_next_token()? {
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(token) => token,
        };

        let mutable = match second_token {
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
        };

        let next_token = match self.lexer.get_next_token()? {
            None => return Err(SyntaxError::NoIdentifierInLetStatement),
            Some(token) => token,
        };

        let identifier = match next_token {
            Token::Identifier(identifier) => identifier,
            _ => return Err(SyntaxError::NoIdentifierInLetStatement),
        };

        Ok(Some(LetStatement {
            identifier,
            mutable,
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

        match second_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::Assignment => {}
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
            None => return Err(SyntaxError::NoIdentifierInAssignmentStatement),
            Some(expression) => expression,
        };

        let assignment_statement = AssignmentStatement {
            identifier,
            expression,
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
}
