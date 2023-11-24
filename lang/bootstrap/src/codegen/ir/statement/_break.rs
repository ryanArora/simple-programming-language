use crate::{
    ast::statement::BreakStatement,
    codegen::ir::{IRBranchStatement, IRState, IRStatement, IRWalkable},
    syntax_error::SyntaxError,
};

impl IRWalkable for BreakStatement {
    type Output = ();

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
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
}
