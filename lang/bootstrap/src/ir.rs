use std::collections::HashMap;

use crate::{
    ast::{
        block::Block,
        expression::{BinaryOperation, BinaryOperationType, Expression, Literal},
        statement::{AssignmentStatement, Statement},
    },
    syntax_error::SyntaxError,
};

#[derive(Debug, PartialEq)]
pub enum IRStatement {
    LoadImmediate(IRLoadImmediateStatement),
    Add(IRAddStatement),
}

#[derive(Debug, PartialEq)]
pub struct IRLoadImmediateStatement {
    rd: u32,
    imm: u64,
}

#[derive(Debug, PartialEq)]
pub struct IRAddStatement {
    rd: u32,
    rs1: u32,
    rs2: u32,
}

#[derive(Debug)]
struct IRState<'a> {
    statements: Vec<IRStatement>,
    symbols: HashMap<&'a str, u32>,
    current_register: u32,
}

pub fn get_ir<'a>(program: &'a Block) -> Result<Vec<IRStatement>, SyntaxError> {
    let mut ir = IRState {
        statements: vec![],
        symbols: HashMap::new(),
        current_register: 0,
    };

    walker_texas_ranger_zzz(&mut ir, program)?;
    Ok(ir.statements)
}

fn walker_texas_ranger_zzz<'a>(ir: &mut IRState<'a>, block: &'a Block) -> Result<(), SyntaxError> {
    for statement in &block.statements {
        walk_statement(ir, statement)?;
    }

    Ok(())
}

fn walk_statement<'a>(ir: &mut IRState<'a>, statement: &'a Statement) -> Result<(), SyntaxError> {
    match statement {
        Statement::LetStatement(_) => unimplemented!(),

        Statement::Assignment(assignment_statement) => {
            walk_assignment_statement(ir, &assignment_statement)
        }

        Statement::IfStatement(_) => unimplemented!(),
        Statement::BreakStatement => unimplemented!(),
        Statement::ContinueStatement => unimplemented!(),
        Statement::LoopStatement(_) => unimplemented!(),
        Statement::WhileStatement(_) => unimplemented!(),
        Statement::Expression(_) => unimplemented!(),
        Statement::EmptyStatement => Ok(()),
    }
}

fn walk_assignment_statement<'a>(
    ir: &mut IRState<'a>,
    assignment_statement: &'a AssignmentStatement,
) -> Result<(), SyntaxError> {
    walk_expression(ir, &assignment_statement.expression)?;

    ir.symbols.insert(
        assignment_statement.identifier.as_str(),
        ir.current_register,
    );

    Ok(())
}

fn walk_expression<'a>(
    ir: &mut IRState<'a>,
    expression: &'a Expression,
) -> Result<u32, SyntaxError> {
    match expression {
        Expression::BinaryOperation(binop) => walk_binary_operation(ir, binop),
        Expression::UnaryOperation(_) => unimplemented!(),
        Expression::Literal(literal) => walk_literal(ir, literal),
        Expression::Identifier(identifier) => walk_identifier(ir, identifier),
    }
}

fn walk_literal<'a>(ir: &mut IRState<'a>, literal: &'a Literal) -> Result<u32, SyntaxError> {
    match literal {
        Literal::StringLiteral(_) => unimplemented!(),
        Literal::IntegerLiteral(int) => walk_integer_literal(ir, *int),
    }
}

fn walk_binary_operation<'a>(
    ir: &mut IRState<'a>,
    binary_operation: &'a BinaryOperation,
) -> Result<u32, SyntaxError> {
    let left_register = walk_expression(ir, &binary_operation.left_expression)?;
    let right_register = walk_expression(ir, &binary_operation.right_expression)?;

    let ir_statement: IRStatement = match binary_operation.operation_type {
        BinaryOperationType::Add => IRStatement::Add(IRAddStatement {
            rd: ir.current_register + 1,
            rs1: left_register,
            rs2: right_register,
        }),
        BinaryOperationType::Subtract => unimplemented!(),
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
        BinaryOperationType::BitwiseAnd => unimplemented!(),
        BinaryOperationType::BitwiseOr => unimplemented!(),
        BinaryOperationType::BitwiseXor => unimplemented!(),
        BinaryOperationType::LeftShift => unimplemented!(),
        BinaryOperationType::RightShift => unimplemented!(),
    };

    ir.statements.push(ir_statement);
    ir.current_register += 1;

    Ok(ir.current_register)
}

fn walk_integer_literal<'a>(
    ir: &mut IRState<'a>,
    integer_literal: u64,
) -> Result<u32, SyntaxError> {
    ir.current_register += 1;

    ir.statements
        .push(IRStatement::LoadImmediate(IRLoadImmediateStatement {
            rd: ir.current_register,
            imm: integer_literal,
        }));

    Ok(ir.current_register)
}

fn walk_identifier<'a>(ir: &mut IRState<'a>, identifier: &'a str) -> Result<u32, SyntaxError> {
    let register = *ir
        .symbols
        .get(identifier)
        .ok_or(SyntaxError::UndefinedReference)?;

    Ok(register)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            block::Block,
            expression::{BinaryOperation, BinaryOperationType, Expression, Literal},
            statement::{AssignmentStatement, Statement},
        },
        ir::{IRAddStatement, IRLoadImmediateStatement, IRStatement},
    };

    use super::get_ir;

    #[test]
    fn test_get_ir() {
        let stmt = Statement::Assignment(AssignmentStatement {
            identifier: "x".to_string(),
            expression: Expression::BinaryOperation(BinaryOperation {
                operation_type: BinaryOperationType::Add,
                left_expression: Box::new(Expression::Literal(Literal::IntegerLiteral(1))),
                right_expression: Box::new(Expression::Literal(Literal::IntegerLiteral(2))),
            }),
        });

        let program = Block {
            statements: vec![stmt],
        };

        let ir = get_ir(&program).unwrap();

        assert_eq!(
            ir,
            vec![
                IRStatement::LoadImmediate(IRLoadImmediateStatement { rd: 1, imm: 1 },),
                IRStatement::LoadImmediate(IRLoadImmediateStatement { rd: 2, imm: 2 },),
                IRStatement::Add(IRAddStatement {
                    rd: 3,
                    rs1: 1,
                    rs2: 2,
                },),
            ]
        );
    }
}
