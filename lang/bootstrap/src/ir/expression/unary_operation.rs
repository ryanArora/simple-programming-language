use crate::{
    ast::expression::{UnaryOperation, UnaryOperationType},
    ir::{IRImmediateStatement, IRRegisterStatement, IRState, IRStatement, IRWalkable},
    syntax_error::SyntaxError,
};

impl IRWalkable for UnaryOperation {
    type Output = u32;

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
        let expression_register = self.expression.walk_ir(ir)?;

        match self.operation_type {
            UnaryOperationType::LogicalNot => unimplemented!(),
            UnaryOperationType::BitwiseNot => {
                let tmp_register = ir.current_register + 1;
                let rd = tmp_register + 1;
                ir.current_register = rd;

                ir.statements
                    .push(IRStatement::LoadImmediate(IRImmediateStatement {
                        rd: tmp_register,
                        imm: 0xFFFFFFFFFFFFFFFF,
                    }));

                ir.statements.push(IRStatement::Xor(IRRegisterStatement {
                    rd,
                    rs1: expression_register,
                    rs2: tmp_register,
                }));

                Ok(rd)
            }
            UnaryOperationType::Plus => Ok(expression_register),
            UnaryOperationType::Minus => {
                let tmp_register = ir.current_register + 1;
                let rd = tmp_register + 1;
                ir.current_register = rd;

                ir.statements
                    .push(IRStatement::LoadImmediate(IRImmediateStatement {
                        rd: tmp_register,
                        imm: 0,
                    }));

                ir.statements
                    .push(IRStatement::Subtract(IRRegisterStatement {
                        rd,
                        rs1: tmp_register,
                        rs2: expression_register,
                    }));

                Ok(rd)
            }
        }
    }
}
