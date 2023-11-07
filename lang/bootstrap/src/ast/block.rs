use crate::{
    ast::statement::Statement,
    lexer::{SimpleToken, Token},
    parser::Parser,
    syntax_error::SyntaxError,
};

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Parser<'_> {
    pub fn get_next_block(&mut self) -> Result<Option<Block>, SyntaxError> {
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
                SimpleToken::LBrace => {}
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

        let mut statements: Vec<Statement> = vec![];
        while let Some(statement) = self.get_next_statement()? {
            statements.push(statement);
        }

        let last_token = match self.lexer.get_next_token()? {
            None => {
                return Err(SyntaxError::UnmatchedBrace);
            }
            Some(token) => token,
        };

        match last_token {
            Token::SimpleToken(simple_token) => match simple_token {
                SimpleToken::RBrace => {}
                _ => {
                    return Err(SyntaxError::UnmatchedBrace);
                }
            },
            _ => {
                return Err(SyntaxError::UnmatchedBrace);
            }
        }

        Ok(Some(Block { statements }))
    }
}
