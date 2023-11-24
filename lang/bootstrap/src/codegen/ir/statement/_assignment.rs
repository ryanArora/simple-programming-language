use crate::{
    ast::statement::AssignmentStatement,
    codegen::ir::{
        get_identifier_register, IRImmediateStatement, IRRegisterStatement, IRState, IRStatement,
        IRWalkable,
    },
    syntax_error::SyntaxError,
};

impl IRWalkable for AssignmentStatement {
    type Output = ();

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
        let rd = match get_identifier_register(ir.scope.clone(), &self.identifier) {
            None => return Err(SyntaxError::AssignedUndeclaredVariable),
            Some(symbol) => symbol,
        };

        let rs1 = self.expression.walk_ir(ir)?;

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
}
