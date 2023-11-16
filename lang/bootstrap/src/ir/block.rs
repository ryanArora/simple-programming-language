use std::collections::HashMap;

use super::{statement::walk_statement, IRState, Scope};
use crate::{ast::block::Block, syntax_error::SyntaxError};

pub fn walk_block<'a>(ir: &mut IRState<'a>, block: &'a Block) -> Result<(), SyntaxError> {
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
