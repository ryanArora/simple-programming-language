use crate::parser;
use parser::Ast;

pub fn analyze(ast: Ast) -> Result<(), AnalyzeError> {
    Ok(())
}

#[derive(Debug)]
pub enum AnalyzeError {}
