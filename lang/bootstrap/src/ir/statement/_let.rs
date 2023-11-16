use crate::{
    ast::statement::LetStatement,
    ir::{expression::walk_expression, IRState},
    syntax_error::SyntaxError,
};

pub fn walk_let_statement<'a>(
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
