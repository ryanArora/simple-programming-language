use crate::{
    ir::{IRBranchStatement, IRState, IRStatement},
    syntax_error::SyntaxError,
};

pub fn walk_break_statement<'a>(ir: &mut IRState<'a>) -> Result<(), SyntaxError> {
    match ir.current_loop_break_label {
        Some(current_loop_break_label) => {
            ir.statements.push(IRStatement::Branch(IRBranchStatement {
                label: current_loop_break_label,
            }));

            Ok(())
        }
        None => Err(SyntaxError::BreakStatementOutsideLoop),
    }
}
