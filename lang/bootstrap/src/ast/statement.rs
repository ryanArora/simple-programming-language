use crate::{
    ast::expression::Expression,
    lexer::{Lexer, SimpleToken, Token},
    parser::Parser,
    syntax_error::SyntaxError,
};

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
    pub fn get_next_statement(&mut self) -> Result<Option<Statement>, SyntaxError> {
        if let Some(statement) = self.get_next_assignment_statement()? {
            return Ok(Some(Statement::Assignment(statement)));
        }

        Ok(None)
    }

    fn get_next_assignment_statement(
        &mut self,
    ) -> Result<Option<AssignmentStatement>, SyntaxError> {
        let old_lexer = self.lexer.clone();

        let first_token_option = self.lexer.get_next_token()?;
        if first_token_option.is_none() {
            self.lexer = old_lexer;
            return Ok(None);
        }
        let first_token = first_token_option.unwrap();

        let identifier: String;
        match first_token {
            Token::Identifier(identifier_inner) => {
                identifier = identifier_inner;
            }
            _ => {
                self.lexer = old_lexer;
                return Ok(None);
            }
        }

        let second_token_option = self.lexer.get_next_token()?;
        if second_token_option.is_none() {
            self.lexer = old_lexer;
            return Ok(None);
        }
        let second_token = second_token_option.unwrap();

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

        let expression_option = self.get_next_expression()?;
        if expression_option.is_none() {
            self.lexer = old_lexer;
            return Ok(None);
        }
        let expression = expression_option.unwrap();

        let assignment_statement = AssignmentStatement {
            identifier,
            expression,
        };

        Ok(Some(assignment_statement))
    }
}
