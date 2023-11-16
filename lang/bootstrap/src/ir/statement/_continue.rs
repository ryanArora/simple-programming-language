use crate::{
    ir::{IRBranchStatement, IRState, IRStatement},
    syntax_error::SyntaxError,
};

pub fn walk_continue_statement<'a>(ir: &mut IRState<'a>) -> Result<(), SyntaxError> {
    match ir.current_loop_continue_label {
        Some(current_loop_continue_label) => {
            ir.statements.push(IRStatement::Branch(IRBranchStatement {
                label: current_loop_continue_label,
            }));

            Ok(())
        }
        None => Err(SyntaxError::ContinueStatementOutsideLoop),
    }
}
