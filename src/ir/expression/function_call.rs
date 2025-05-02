use crate::{
    ast::expression::FunctionCall,
    ir::{IRState, IRStatement, IRWalkable, Register},
    syntax_error::SyntaxError,
};

impl IRWalkable for FunctionCall {
    type Output = u32;

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
        // TODO: Temporarily hardcode print
        if self.function_name == "print" {
            let argument = self.argument.walk_ir(ir)?;
            ir.statements.push(IRStatement::Print {
                rs1: Register(argument),
            });
            Ok(1)
        } else {
            unimplemented!()
        }
    }
}
