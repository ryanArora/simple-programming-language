use crate::{
    ast::statement::WhileStatement,
    ir::{
        block::walk_block, expression::walk_expression, IRBranchStatement,
        IRConditionalBranchStatement, IRLabelStatement, IRState, IRStatement,
    },
    syntax_error::SyntaxError,
};

pub fn walk_while_statement<'a>(
    ir: &mut IRState<'a>,
    while_statement: &'a WhileStatement,
) -> Result<(), SyntaxError> {
    let loop_start_label = ir.current_label + 1;
    let loop_continue_label = loop_start_label + 1;
    let loop_break_label = loop_continue_label + 1;
    ir.current_label = loop_break_label;

    ir.statements.push(IRStatement::Label(IRLabelStatement {
        label: loop_start_label,
    }));

    let condition_register = walk_expression(ir, &while_statement.condition)?;

    ir.statements
        .push(IRStatement::BranchZero(IRConditionalBranchStatement {
            register: condition_register,
            label: loop_break_label,
        }));

    let old_loop_continue_label = ir.current_loop_continue_label;
    let old_loop_break_label = ir.current_loop_break_label;
    ir.current_loop_continue_label = Some(loop_continue_label);
    ir.current_loop_break_label = Some(loop_break_label);

    walk_block(ir, &while_statement.block)?;

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
