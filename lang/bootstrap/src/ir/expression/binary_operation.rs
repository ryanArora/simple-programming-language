use crate::{
    ast::expression::{BinaryOperation, BinaryOperationType},
    ir::{IRRegisterStatement, IRState, IRStatement, IRWalkable},
    syntax_error::SyntaxError,
};

impl IRWalkable for BinaryOperation {
    type Output = u32;

    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError> {
        let left = self.left_expression.walk_ir(ir)?;
        let right = self.right_expression.walk_ir(ir)?;

        match self.operation_type {
            BinaryOperationType::Add => push_irstatement_add(ir, left, right),
            BinaryOperationType::Subtract => push_irstatement_sub(ir, left, right),
            BinaryOperationType::Multiply => unimplemented!(),
            BinaryOperationType::Divide => unimplemented!(),
            BinaryOperationType::Modulus => unimplemented!(),
            BinaryOperationType::Exponentiate => unimplemented!(),
            BinaryOperationType::Equal => unimplemented!(),
            BinaryOperationType::NotEqual => unimplemented!(),
            BinaryOperationType::GreaterEqual => unimplemented!(),
            BinaryOperationType::LessEqual => unimplemented!(),
            BinaryOperationType::Greater => unimplemented!(),
            BinaryOperationType::Less => unimplemented!(),
            BinaryOperationType::LogicalAnd => unimplemented!(),
            BinaryOperationType::LogicalOr => unimplemented!(),
            BinaryOperationType::BitwiseAnd => push_irstatement_and(ir, left, right),
            BinaryOperationType::BitwiseOr => push_irstatement_or(ir, left, right),
            BinaryOperationType::BitwiseXor => push_irstatement_xor(ir, left, right),
            BinaryOperationType::LeftShift => push_irstatement_leftshift(ir, left, right),
            BinaryOperationType::RightShift => push_irstatement_rightshift(ir, left, right),
        }
    }
}

fn push_irstatement_add<'a>(
    ir: &mut IRState<'a>,
    left_register: u32,
    right_register: u32,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    let ir_statement = IRStatement::Add(IRRegisterStatement {
        rd: ir.current_register,
        rs1: left_register,
        rs2: right_register,
    });

    ir.statements.push(ir_statement);
    Ok(ir.current_register)
}

fn push_irstatement_sub<'a>(
    ir: &mut IRState<'a>,
    left_register: u32,
    right_register: u32,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    let ir_statement = IRStatement::Subtract(IRRegisterStatement {
        rd: ir.current_register,
        rs1: left_register,
        rs2: right_register,
    });

    ir.statements.push(ir_statement);
    Ok(ir.current_register)
}

fn push_irstatement_and<'a>(
    ir: &mut IRState<'a>,
    left_register: u32,
    right_register: u32,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    let ir_statement = IRStatement::And(IRRegisterStatement {
        rd: ir.current_register,
        rs1: left_register,
        rs2: right_register,
    });

    ir.statements.push(ir_statement);
    Ok(ir.current_register)
}

fn push_irstatement_or<'a>(
    ir: &mut IRState<'a>,
    left_register: u32,
    right_register: u32,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    let ir_statement = IRStatement::Or(IRRegisterStatement {
        rd: ir.current_register,
        rs1: left_register,
        rs2: right_register,
    });

    ir.statements.push(ir_statement);
    Ok(ir.current_register)
}

fn push_irstatement_xor<'a>(
    ir: &mut IRState<'a>,
    left_register: u32,
    right_register: u32,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    let ir_statement = IRStatement::Xor(IRRegisterStatement {
        rd: ir.current_register,
        rs1: left_register,
        rs2: right_register,
    });

    ir.statements.push(ir_statement);
    Ok(ir.current_register)
}

fn push_irstatement_leftshift<'a>(
    ir: &mut IRState<'a>,
    left_register: u32,
    right_register: u32,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    let ir_statement = IRStatement::LeftShift(IRRegisterStatement {
        rd: ir.current_register,
        rs1: left_register,
        rs2: right_register,
    });

    ir.statements.push(ir_statement);
    Ok(ir.current_register)
}

fn push_irstatement_rightshift<'a>(
    ir: &mut IRState<'a>,
    left_register: u32,
    right_register: u32,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    let ir_statement = IRStatement::RightShift(IRRegisterStatement {
        rd: ir.current_register,
        rs1: left_register,
        rs2: right_register,
    });

    ir.statements.push(ir_statement);
    Ok(ir.current_register)
}
