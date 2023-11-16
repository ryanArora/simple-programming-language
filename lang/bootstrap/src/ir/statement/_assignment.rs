use crate::{
    ast::statement::AssignmentStatement,
    ir::{
        expression::walk_expression, get_identifier_register, IRImmediateStatement,
        IRRegisterStatement, IRState, IRStatement,
    },
    syntax_error::SyntaxError,
};

pub fn walk_assignment_statement<'a>(
    ir: &mut IRState<'a>,
    assignment_statement: &'a AssignmentStatement,
) -> Result<(), SyntaxError> {
    let rd = match get_identifier_register(ir.scope.clone(), &assignment_statement.identifier) {
        None => return Err(SyntaxError::AssignedUndeclaredVariable),
        Some(symbol) => symbol,
    };

    let rs1 = walk_expression(ir, &assignment_statement.expression)?;

    ir.current_register += 1;

    ir.statements
        .push(IRStatement::LoadImmediate(IRImmediateStatement {
            rd: ir.current_register,
            imm: 0,
        }));

    ir.statements.push(IRStatement::Add(IRRegisterStatement {
        rd,
        rs1,
        rs2: ir.current_register,
    }));

    Ok(())
}
