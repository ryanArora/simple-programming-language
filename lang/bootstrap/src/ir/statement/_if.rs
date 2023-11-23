use crate::{
    ast::statement::IfStatement,
    ir::{
        IRBranchStatement, IRConditionalBranchStatement, IRLabelStatement, IRState, IRStatement,
        IRWalkable,
    },
    syntax_error::SyntaxError,
};

impl IRWalkable for IfStatement {
    type Output = ();

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
        //
        // Allocate labels
        //
        let if_label = ir.current_label + 1;
        let first_else_if_label = if_label + 1;
        let else_label = first_else_if_label + u32::try_from(self.else_if.len()).unwrap();
        let done_label = else_label + 1;
        ir.current_label = done_label;

        //
        // CONDITIONS
        //

        // If
        let if_condition = self._if.condition.walk_ir(ir)?;

        ir.statements
            .push(IRStatement::BranchNotZero(IRConditionalBranchStatement {
                register: if_condition,
                label: if_label,
            }));

        // Else if

        let mut current_else_if_label = first_else_if_label;
        for else_if in &self.else_if {
            let else_if_condition = else_if.condition.walk_ir(ir)?;

            ir.statements
                .push(IRStatement::BranchNotZero(IRConditionalBranchStatement {
                    register: else_if_condition,
                    label: current_else_if_label,
                }));

            current_else_if_label += 1;
        }

        // Branch to else or done label

        match self._else {
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
        self._if.block.walk_ir(ir)?;
        ir.statements
            .push(IRStatement::Branch(IRBranchStatement { label: done_label }));

        // Else if blocks
        let mut current_else_if_label = first_else_if_label;
        for else_if in &self.else_if {
            ir.statements.push(IRStatement::Label(IRLabelStatement {
                label: current_else_if_label,
            }));

            else_if.block.walk_ir(ir)?;

            ir.statements
                .push(IRStatement::Branch(IRBranchStatement { label: done_label }));

            current_else_if_label += 1;
        }

        // Else block
        ir.statements
            .push(IRStatement::Label(IRLabelStatement { label: else_label }));

        match &self._else {
            None => {}
            Some(block) => block.walk_ir(ir)?,
        }

        // Done label
        ir.statements
            .push(IRStatement::Label(IRLabelStatement { label: done_label }));

        Ok(())
    }
}
