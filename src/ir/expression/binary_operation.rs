use crate::{
    ast::expression::{BinaryOperation, BinaryOperationType},
    ir::{IRState, IRStatement, IRWalkable, Register},
    syntax_error::SyntaxError,
};

impl IRWalkable for BinaryOperation {
    type Output = u32;

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
        let left = self.left_expression.walk_ir(ir)?;
        let right = self.right_expression.walk_ir(ir)?;

        match self.operation_type {
            BinaryOperationType::Add => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Add {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::Subtract => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Subtract {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::Multiply => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Multiply {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::Divide => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Divide {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::Modulus => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Modulus {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::Exponentiate => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Exponentiate {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::Equal => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Equal {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::NotEqual => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::NotEqual {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::GreaterEqual => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::GreaterEqual {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::LessEqual => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::LessEqual {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::Greater => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Greater {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::Less => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::Less {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::LogicalAnd => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::LogicalAnd {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::LogicalOr => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::LogicalOr {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::BitwiseAnd => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::BitwiseAnd {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::BitwiseOr => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::BitwiseOr {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::BitwiseXor => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::BitwiseXor {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::LeftShift => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::LeftShift {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
            BinaryOperationType::RightShift => {
                ir.current_register += 1;
                ir.statements.push(IRStatement::RightShift {
                    rd: Register(ir.current_register),
                    rs1: Register(left),
                    rs2: Register(right),
                });
                Ok(ir.current_register)
            }
        }
    }
}
