use crate::{
    ast::statement::IfStatement,
    ir::{
        block::walk_block, expression::walk_expression, IRBranchStatement,
        IRConditionalBranchStatement, IRLabelStatement, IRState, IRStatement,
    },
    syntax_error::SyntaxError,
};

pub fn walk_if_statement<'a>(
    ir: &mut IRState<'a>,
    if_statement: &'a IfStatement,
) -> Result<(), SyntaxError> {
    //
    // Allocate labels
    //
    let if_label = ir.current_label + 1;
    let first_else_if_label = if_label + 1;
    let else_label = first_else_if_label + u32::try_from(if_statement.else_if.len()).unwrap();
    let done_label = else_label + 1;
    ir.current_label = done_label;

    //
    // CONDITIONS
    //

    // If
    let if_condition = walk_expression(ir, &if_statement._if.condition)?;

    ir.statements
        .push(IRStatement::BranchNotZero(IRConditionalBranchStatement {
            register: if_condition,
            label: if_label,
        }));

    // Else if
    let mut current_else_if_label = first_else_if_label;
    for else_if in &if_statement.else_if {
        let else_if_condition = walk_expression(ir, &else_if.condition)?;

        ir.statements
            .push(IRStatement::BranchNotZero(IRConditionalBranchStatement {
                register: else_if_condition,
                label: current_else_if_label,
            }));

        current_else_if_label += 1;
    }

    // Branch to else or done label

    match if_statement._else {
        Some(_) => ir
            .statements
            .push(IRStatement::Branch(IRBranchStatement { label: else_label })),
        None => ir
            .statements
            .push(IRStatement::Branch(IRBranchStatement { label: done_label })),
    }

    //
    // BLOCKS
    //

    // If block
    ir.statements
        .push(IRStatement::Label(IRLabelStatement { label: if_label }));
    walk_block(ir, &if_statement._if.block)?;
    ir.statements
        .push(IRStatement::Branch(IRBranchStatement { label: done_label }));

    // Else if blocks
    let mut current_else_if_label = first_else_if_label;
    for else_if in &if_statement.else_if {
        ir.statements.push(IRStatement::Label(IRLabelStatement {
            label: current_else_if_label,
        }));

        walk_block(ir, &else_if.block)?;

        ir.statements
            .push(IRStatement::Branch(IRBranchStatement { label: done_label }));

        current_else_if_label += 1;
    }

    // Else block
    ir.statements
        .push(IRStatement::Label(IRLabelStatement { label: else_label }));

    match &if_statement._else {
        None => {}
        Some(block) => walk_block(ir, block)?,
    }

    // Done label
    ir.statements
        .push(IRStatement::Label(IRLabelStatement { label: done_label }));

    Ok(())
}
