mod _assignment;
mod _break;
mod _continue;
mod _if;
mod _let;
mod _loop;
mod _while;

use self::{
    _assignment::walk_assignment_statement, _break::walk_break_statement,
    _continue::walk_continue_statement, _if::walk_if_statement, _let::walk_let_statement,
    _loop::walk_loop_statement, _while::walk_while_statement,
};
use super::{expression::walk_expression, IRState};
use crate::{ast::statement::Statement, syntax_error::SyntaxError};

pub fn walk_statement<'a>(
    ir: &mut IRState<'a>,
    statement: &'a Statement,
) -> Result<(), SyntaxError> {
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
