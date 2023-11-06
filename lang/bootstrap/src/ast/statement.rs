use crate::{
    ast::expression::Expression,
    lexer::{SimpleToken, Token},
    parser::Parser,
    syntax_error::SyntaxError,
};

#[derive(Debug)]
pub enum Statement {
    Assignment(AssignmentStatement),
    Expression(Expression),
    EmptyStatement,
}

#[derive(Debug)]
pub struct AssignmentStatement {
    identifier: String,
    expression: Expression,
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

        if let Some(statement) = self.get_next_assignment_statement()? {
            next_statement = Some(Statement::Assignment(statement));
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
            None => {
                self.lexer = old_lexer;
                return Ok(None);
            }
            Some(expression) => expression,
        };

        let assignment_statement = AssignmentStatement {
            identifier,
            expression,
        };

        Ok(Some(assignment_statement))
    }
}
