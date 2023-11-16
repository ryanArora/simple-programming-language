use core::fmt;
use std::{collections::HashMap, fmt::Display};

use crate::{
    ast::{
        block::Block,
        expression::{
            BinaryOperation, BinaryOperationType, Expression, Literal, UnaryOperation,
            UnaryOperationType,
        },
        statement::{
            AssignmentStatement, IfStatement, LetStatement, LoopStatement, Statement,
            WhileStatement,
        },
    },
    syntax_error::SyntaxError,
};

#[derive(Debug, PartialEq)]
pub enum IRStatement {
    LoadImmediate(IRImmediateStatement),
    Add(IRRegisterStatement),
    Subtract(IRRegisterStatement),
    And(IRRegisterStatement),
    Or(IRRegisterStatement),
    Xor(IRRegisterStatement),
    LeftShift(IRRegisterStatement),
    RightShift(IRRegisterStatement),
    Branch(IRBranchStatement),
    BranchNotZero(IRConditionalBranchStatement),
    BranchZero(IRConditionalBranchStatement),
    Label(IRLabelStatement),
}

impl Display for IRStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRStatement::LoadImmediate(s) => write!(f, "li t{}, {}", s.rd, s.imm),
            IRStatement::Add(s) => write!(f, "add t{}, t{}, t{}", s.rd, s.rs1, s.rs2),
            IRStatement::Subtract(s) => write!(f, "sub t{}, t{}, t{}", s.rd, s.rs1, s.rs2),
            IRStatement::And(s) => write!(f, "and t{}, t{}, t{}", s.rd, s.rs1, s.rs2),
            IRStatement::Or(s) => write!(f, "or t{}, t{}, t{}", s.rd, s.rs1, s.rs2),
            IRStatement::Xor(s) => write!(f, "xor t{}, t{}, t{}", s.rd, s.rs1, s.rs2),
            IRStatement::LeftShift(s) => write!(f, "sll t{}, t{}, t{}", s.rd, s.rs1, s.rs2),
            IRStatement::RightShift(s) => write!(f, "srl t{}, t{}, t{}", s.rd, s.rs1, s.rs2),
            IRStatement::Branch(s) => write!(f, "j L{}", s.label),
            IRStatement::BranchNotZero(s) => write!(f, "bnz t{}, L{}", s.register, s.label),
            IRStatement::BranchZero(s) => write!(f, "bz t{}, L{}", s.register, s.label),
            IRStatement::Label(s) => write!(f, "L{}:", s.label),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IRImmediateStatement {
    rd: u32,
    imm: u64,
}

#[derive(Debug, PartialEq)]
pub struct IRRegisterStatement {
    rd: u32,
    rs1: u32,
    rs2: u32,
}

#[derive(Debug, PartialEq)]
pub struct IRBranchStatement {
    label: u32,
}

#[derive(Debug, PartialEq)]
pub struct IRConditionalBranchStatement {
    register: u32,
    label: u32,
}

#[derive(Debug, PartialEq)]
pub struct IRLabelStatement {
    label: u32,
}

#[derive(Debug)]
struct IRState<'a> {
    statements: Vec<IRStatement>,
    scope: Option<Scope<'a>>,
    current_register: u32,
    current_label: u32,
    current_loop_continue_label: Option<u32>,
    current_loop_break_label: Option<u32>,
}
#[derive(Debug, Clone)]
struct Scope<'a> {
    previous_scope: Option<Box<Scope<'a>>>,
    symbols: HashMap<&'a str, u32>,
}

fn get_identifier_register<'a>(scope: Option<Scope>, identifier: &'a str) -> Option<u32> {
    let mut current_scope_option = scope;
    loop {
        if current_scope_option.is_none() {
            break;
        }

        let current_scope = current_scope_option.unwrap();

        match current_scope.symbols.get(identifier) {
            Some(symbol) => return Some(*symbol),
            None => {}
        }

        current_scope_option = match current_scope.previous_scope {
            Some(scope) => Some(*scope),
            None => None,
        };
    }

    None
}

pub fn get_ir<'a>(program: &'a Block) -> Result<Vec<IRStatement>, SyntaxError> {
    let mut ir = IRState {
        statements: vec![],
        scope: None,
        current_register: 0,
        current_label: 0,
        current_loop_continue_label: None,
        current_loop_break_label: None,
    };

    walk_block(&mut ir, program)?;
    Ok(ir.statements)
}

fn walk_block<'a>(ir: &mut IRState<'a>, block: &'a Block) -> Result<(), SyntaxError> {
    let old_scope = ir.scope.clone();

    ir.scope = match &ir.scope {
        None => Some(Scope {
            previous_scope: None,
            symbols: HashMap::new(),
        }),
        Some(scope) => Some(Scope {
            previous_scope: Some(Box::new(scope.clone())),
            symbols: HashMap::new(),
        }),
    };

    for statement in &block.statements {
        walk_statement(ir, statement)?;
    }

    ir.scope = old_scope;

    Ok(())
}

fn walk_statement<'a>(ir: &mut IRState<'a>, statement: &'a Statement) -> Result<(), SyntaxError> {
    match statement {
        Statement::LetStatement(let_stmt) => walk_let_statement(ir, let_stmt),
        Statement::Assignment(assignment_stmt) => walk_assignment_statement(ir, assignment_stmt),
        Statement::IfStatement(if_stmt) => walk_if_statement(ir, if_stmt),
        Statement::BreakStatement => walk_break_statement(ir),
        Statement::ContinueStatement => walk_continue_statement(ir),
        Statement::LoopStatement(loop_stmt) => walk_loop_statement(ir, loop_stmt),
        Statement::WhileStatement(while_stmt) => walk_while_statement(ir, while_stmt),
        Statement::Expression(expr) => match walk_expression(ir, expr) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Statement::EmptyStatement => Ok(()),
    }
}

fn walk_let_statement<'a>(
    ir: &mut IRState<'a>,
    let_statement: &'a LetStatement,
) -> Result<(), SyntaxError> {
    let register = match &let_statement.expression {
        None => {
            ir.current_register += 1;
            ir.current_register
        }
        Some(expression) => walk_expression(ir, expression)?,
    };

    ir.scope
        .as_mut()
        .unwrap()
        .symbols
        .insert(&let_statement.identifier, register);

    Ok(())
}

fn walk_assignment_statement<'a>(
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

fn walk_expression<'a>(
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

fn walk_if_statement<'a>(
    ir: &mut IRState<'a>,
    if_statement: &'a IfStatement,
) -> Result<(), SyntaxError> {
    //
    // Allocate labels
    //
    let if_label = ir.current_label + 1;
    let first_else_if_label = if_label + 1;
    let else_label = first_else_if_label + u32::try_from(if_statement.else_if.len()).unwrap();
    let done_label = else_label + 1;
    ir.current_label = done_label;

    //
    // CONDITIONS
    //

    // If
    let if_condition = walk_expression(ir, &if_statement._if.condition)?;

    ir.statements
        .push(IRStatement::BranchNotZero(IRConditionalBranchStatement {
            register: if_condition,
            label: if_label,
        }));

    // Else if
    let mut current_else_if_label = first_else_if_label;
    for else_if in &if_statement.else_if {
        let else_if_condition = walk_expression(ir, &else_if.condition)?;

        ir.statements
            .push(IRStatement::BranchNotZero(IRConditionalBranchStatement {
                register: else_if_condition,
                label: current_else_if_label,
            }));

        current_else_if_label += 1;
    }

    // Branch to else or done label

    match if_statement._else {
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
    walk_block(ir, &if_statement._if.block)?;
    ir.statements
        .push(IRStatement::Branch(IRBranchStatement { label: done_label }));

    // Else if blocks
    let mut current_else_if_label = first_else_if_label;
    for else_if in &if_statement.else_if {
        ir.statements.push(IRStatement::Label(IRLabelStatement {
            label: current_else_if_label,
        }));

        walk_block(ir, &else_if.block)?;

        ir.statements
            .push(IRStatement::Branch(IRBranchStatement { label: done_label }));

        current_else_if_label += 1;
    }

    // Else block
    ir.statements
        .push(IRStatement::Label(IRLabelStatement { label: else_label }));

    match &if_statement._else {
        None => {}
        Some(block) => walk_block(ir, block)?,
    }

    // Done label
    ir.statements
        .push(IRStatement::Label(IRLabelStatement { label: done_label }));

    Ok(())
}

fn walk_loop_statement<'a>(
    ir: &mut IRState<'a>,
    loop_statement: &'a LoopStatement,
) -> Result<(), SyntaxError> {
    let loop_start_label = ir.current_label + 1;
    let loop_continue_label = loop_start_label + 1;
    let loop_break_label = loop_continue_label + 1;
    ir.current_label = loop_break_label;

    ir.statements.push(IRStatement::Label(IRLabelStatement {
        label: loop_start_label,
    }));

    let old_loop_continue_label = ir.current_loop_continue_label;
    let old_loop_break_label = ir.current_loop_break_label;
    ir.current_loop_continue_label = Some(loop_continue_label);
    ir.current_loop_break_label = Some(loop_break_label);

    walk_block(ir, &loop_statement.block)?;

    ir.current_loop_continue_label = old_loop_continue_label;
    ir.current_loop_break_label = old_loop_break_label;

    ir.statements.push(IRStatement::Label(IRLabelStatement {
        label: loop_continue_label,
    }));

    ir.statements.push(IRStatement::Branch(IRBranchStatement {
        label: loop_start_label,
    }));

    ir.statements.push(IRStatement::Label(IRLabelStatement {
        label: loop_break_label,
    }));

    Ok(())
}

fn walk_while_statement<'a>(
    ir: &mut IRState<'a>,
    while_statement: &'a WhileStatement,
) -> Result<(), SyntaxError> {
    let loop_start_label = ir.current_label + 1;
    let loop_continue_label = loop_start_label + 1;
    let loop_break_label = loop_continue_label + 1;
    ir.current_label = loop_break_label;

    ir.statements.push(IRStatement::Label(IRLabelStatement {
        label: loop_start_label,
    }));

    let condition_register = walk_expression(ir, &while_statement.condition)?;

    ir.statements
        .push(IRStatement::BranchZero(IRConditionalBranchStatement {
            register: condition_register,
            label: loop_break_label,
        }));

    let old_loop_continue_label = ir.current_loop_continue_label;
    let old_loop_break_label = ir.current_loop_break_label;
    ir.current_loop_continue_label = Some(loop_continue_label);
    ir.current_loop_break_label = Some(loop_break_label);

    walk_block(ir, &while_statement.block)?;

    ir.current_loop_continue_label = old_loop_continue_label;
    ir.current_loop_break_label = old_loop_break_label;

    ir.statements.push(IRStatement::Label(IRLabelStatement {
        label: loop_continue_label,
    }));

    ir.statements.push(IRStatement::Branch(IRBranchStatement {
        label: loop_start_label,
    }));

    ir.statements.push(IRStatement::Label(IRLabelStatement {
        label: loop_break_label,
    }));

    Ok(())
}

fn walk_continue_statement<'a>(ir: &mut IRState<'a>) -> Result<(), SyntaxError> {
    match ir.current_loop_continue_label {
        Some(current_loop_continue_label) => {
            ir.statements.push(IRStatement::Branch(IRBranchStatement {
                label: current_loop_continue_label,
            }));

            Ok(())
        }
        None => Err(SyntaxError::ContinueStatementOutsideLoop),
    }
}

fn walk_break_statement<'a>(ir: &mut IRState<'a>) -> Result<(), SyntaxError> {
    match ir.current_loop_break_label {
        Some(current_loop_break_label) => {
            ir.statements.push(IRStatement::Branch(IRBranchStatement {
                label: current_loop_break_label,
            }));

            Ok(())
        }
        None => Err(SyntaxError::BreakStatementOutsideLoop),
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

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            block::Block,
            expression::{BinaryOperation, BinaryOperationType, Expression, Literal},
            statement::{AssignmentStatement, Statement},
        },
        ir::{IRImmediateStatement, IRRegisterStatement, IRStatement},
        parser::Parser,
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
                IRStatement::LoadImmediate(IRImmediateStatement { rd: 1, imm: 1 }),
                IRStatement::LoadImmediate(IRImmediateStatement { rd: 2, imm: 2 }),
                IRStatement::Add(IRRegisterStatement {
                    rd: 3,
                    rs1: 1,
                    rs2: 2,
                }),
            ]
        );
    }

    #[test]
    fn test_get_ir_simple_if() {
        let mut parser = Parser::new("if (10 + 20 - 30 | 40 & 50 ^ 60 << 70 >> 80) {};");
        let program = parser.get_ast().unwrap().unwrap();
        let ir = get_ir(&program).unwrap();

        for stmt in ir {
            println!("{}", stmt);
        }
    }

    #[test]
    fn test_get_ir_simple_loop() {
        let mut parser = Parser::new("loop { if 1 + 1 { break; }; };");
        let program = parser.get_ast().unwrap().unwrap();
        let ir = get_ir(&program).unwrap();

        for stmt in ir {
            println!("{}", stmt);
        }
    }

    #[test]
    fn test_get_ir_branched_assignment() {
        let mut parser = Parser::new("let x; if 1 { x = 1; } else { x = 2; };");
        let program = parser.get_ast().unwrap().unwrap();
        let ir = get_ir(&program).unwrap();

        for stmt in ir {
            println!("{}", stmt);
        }
    }
}
