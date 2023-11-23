use crate::{
    ast::statement::WhileStatement,
    ir::{
        IRBranchStatement, IRConditionalBranchStatement, IRLabelStatement, IRState, IRStatement,
        IRWalkable,
    },
    syntax_error::SyntaxError,
};

impl IRWalkable for WhileStatement {
    type Output = ();

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
        let loop_start_label = ir.current_label + 1;
        let loop_continue_label = loop_start_label + 1;
        let loop_break_label = loop_continue_label + 1;
        ir.current_label = loop_break_label;

        ir.statements.push(IRStatement::Label(IRLabelStatement {
            label: loop_start_label,
        }));

        let condition_register = self.condition.walk_ir(ir)?;

        ir.statements
            .push(IRStatement::BranchZero(IRConditionalBranchStatement {
                register: condition_register,
                label: loop_break_label,
            }));

        let old_loop_continue_label = ir.current_loop_continue_label;
        let old_loop_break_label = ir.current_loop_break_label;
        ir.current_loop_continue_label = Some(loop_continue_label);
        ir.current_loop_break_label = Some(loop_break_label);

        self.block.walk_ir(ir)?;

        ir.current_loop_continue_label = old_loop_continue_label;
        ir.current_loop_break_label = old_loop_break_label;

        ir.statements.push(IRStatement::Label(IRLabelStatement {
            label: loop_continue_label,
        }));

        ir.statements.push(IRStatement::Branch(IRBranchStatement {
            label: loop_start_label,
        }));

        ir.statements.push(IRStatement::Label(IRLabelStatement {
            label: loop_break_label,
        }));

        Ok(())
    }
}
