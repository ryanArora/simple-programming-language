use crate::{
    ast::expression::Literal,
    codegen::ir::{IRImmediateStatement, IRState, IRStatement, IRWalkable},
    syntax_error::SyntaxError,
};

impl IRWalkable for Literal {
    type Output = u32;

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
        match self {
            Literal::StringLiteral(_) => unimplemented!(),
            Literal::IntegerLiteral(int) => walk_integer_literal(ir, *int),
        }
    }
}

fn walk_integer_literal<'a>(ir: &'a mut IRState, integer_literal: u64) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    ir.statements
        .push(IRStatement::LoadImmediate(IRImmediateStatement {
            rd: ir.current_register,
            imm: integer_literal,
        }));

    Ok(ir.current_register)
}
