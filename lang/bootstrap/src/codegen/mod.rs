use crate::{ast::block::Block, syntax_error::SyntaxError};

mod ir;
mod register_allocation;

pub fn get_code(program: &Block) -> Result<(), SyntaxError> {
    let mut ir = ir::get_ir(program)?;

    // Optimize here...

    register_allocation::spill_extra_virtual_registers(&mut ir, 8);

    Ok(())
}
