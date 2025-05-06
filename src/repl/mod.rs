use crate::ast::block::Block;
use crate::interp;
use crate::ir;
use crate::parser::Parser;
use std::io::{self, Write};

pub fn start_repl() {
    println!(
        "Welcome to the Simple Programming Language v{}",
        env!("CARGO_PKG_VERSION")
    );

    let mut megablock = Block {
        statements: Vec::new(),
    };

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input_data = String::new();
        io::stdin().read_line(&mut input_data).unwrap();

        if input_data.trim() == "exit" || input_data.trim() == "quit" {
            break;
        }

        let mut p = Parser::new(&input_data);
        let ast = match p.get_ast() {
            Ok(ast) => match ast {
                Some(ast) => ast,
                None => {
                    panic!();
                }
            },
            Err(e) => {
                println!("Error: {:#?}", e);
                continue;
            }
        };

        for stmt in ast.statements {
            megablock.statements.push(stmt);
        }

        let ir = match ir::get_ir(&megablock) {
            Ok(ir) => ir,
            Err(e) => {
                println!("Error: {:#?}", e);
                continue;
            }
        };

        let result = interp::interpret(&ir);
        println!("{}", result);
    }
}
