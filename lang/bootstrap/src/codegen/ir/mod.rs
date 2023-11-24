mod block;
mod expression;
mod statement;

use crate::{ast::block::Block, syntax_error::SyntaxError};
use core::fmt;
use std::{collections::HashMap, fmt::Display};

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
    Push(IRPushStatement),
    Pop(IRPopStatement),
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
            IRStatement::Push(s) => write!(f, "push t{}", s.register),
            IRStatement::Pop(s) => write!(f, "pop t{}", s.register),
        }
    }
}

trait IRWalkable {
    type Output;
    fn walk_ir<'a>(&'a self, ir: &mut IRState<'a>) -> Result<Self::Output, SyntaxError>;
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

#[derive(Debug, PartialEq)]
pub struct IRPushStatement {
    register: u32,
}

#[derive(Debug, PartialEq)]
pub struct IRPopStatement {
    register: u32,
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

pub fn get_ir<'a>(program: &'a Block) -> Result<Vec<IRStatement>, SyntaxError> {
    let mut ir = IRState {
        statements: vec![],
        scope: None,
        current_register: 0,
        current_label: 0,
        current_loop_continue_label: None,
        current_loop_break_label: None,
    };

    program.walk_ir(&mut ir)?;
    Ok(ir.statements)
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

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            block::Block,
            expression::{BinaryOperation, BinaryOperationType, Expression, Literal},
            statement::{AssignmentStatement, Statement},
        },
        codegen::ir::{IRImmediateStatement, IRRegisterStatement, IRStatement},
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
