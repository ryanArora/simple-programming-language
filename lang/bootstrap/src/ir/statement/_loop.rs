use crate::{
    ast::statement::LoopStatement,
    ir::{block::walk_block, IRBranchStatement, IRLabelStatement, IRState, IRStatement},
    syntax_error::SyntaxError,
};

pub fn walk_loop_statement<'a>(
    ir: &mut IRState<'a>,
    loop_statement: &'a LoopStatement,
) -> Result<(), SyntaxError> {
    let loop_start_label = ir.current_label + 1;
    let loop_continue_label = loop_start_label + 1;
    let loop_break_label = loop_continue_label + 1;
    ir.current_label = loop_break_label;

    ir.statements.push(IRStatement::Label(IRLabelStatement {
        label: loop_start_label,
    }));

    let old_loop_continue_label = ir.current_loop_continue_label;
    let old_loop_break_label = ir.current_loop_break_label;
    ir.current_loop_continue_label = Some(loop_continue_label);
    ir.current_loop_break_label = Some(loop_break_label);

    walk_block(ir, &loop_statement.block)?;

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
