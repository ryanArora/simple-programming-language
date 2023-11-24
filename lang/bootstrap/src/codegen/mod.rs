use crate::{ast::block::Block, syntax_error::SyntaxError};

mod ir;

pub fn get_code(program: &Block) -> Result<(), SyntaxError> {
    let ir = ir::get_ir(program)?;

    Ok(())
}
