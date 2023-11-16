use crate::{
    ast::expression::{
        BinaryOperation, BinaryOperationType, Expression, Literal, UnaryOperation,
        UnaryOperationType,
    },
    syntax_error::SyntaxError,
};

use super::{
    get_identifier_register, IRImmediateStatement, IRRegisterStatement, IRState, IRStatement,
};

pub fn walk_expression<'a>(
    ir: &mut IRState<'a>,
    expression: &'a Expression,
) -> Result<u32, SyntaxError> {
    match expression {
        Expression::BinaryOperation(binary_op) => walk_binary_operation(ir, binary_op),
        Expression::UnaryOperation(unary_op) => walk_unary_operation(ir, unary_op),
        Expression::Literal(literal) => walk_literal(ir, literal),
        Expression::Identifier(identifier) => walk_identifier(ir, identifier),
    }
}

fn walk_binary_operation<'a>(
    ir: &mut IRState<'a>,
    binary_operation: &'a BinaryOperation,
) -> Result<u32, SyntaxError> {
    let left = walk_expression(ir, &binary_operation.left_expression)?;
    let right = walk_expression(ir, &binary_operation.right_expression)?;

    match binary_operation.operation_type {
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

fn walk_unary_operation<'a>(
    ir: &mut IRState<'a>,
    unary_operation: &'a UnaryOperation,
) -> Result<u32, SyntaxError> {
    let expression_register = walk_expression(ir, &unary_operation.expression)?;

    match unary_operation.operation_type {
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

fn walk_literal<'a>(ir: &mut IRState<'a>, literal: &'a Literal) -> Result<u32, SyntaxError> {
    match literal {
        Literal::StringLiteral(_) => unimplemented!(),
        Literal::IntegerLiteral(int) => walk_integer_literal(ir, *int),
    }
}

fn walk_integer_literal<'a>(
    ir: &mut IRState<'a>,
    integer_literal: u64,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    ir.statements
        .push(IRStatement::LoadImmediate(IRImmediateStatement {
            rd: ir.current_register,
            imm: integer_literal,
        }));

    Ok(ir.current_register)
}

fn walk_identifier<'a>(ir: &mut IRState<'a>, identifier: &'a str) -> Result<u32, SyntaxError> {
    match get_identifier_register(ir.scope.clone(), identifier) {
        None => Err(SyntaxError::UndefinedReference),
        Some(register) => Ok(register),
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
