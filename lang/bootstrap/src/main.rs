#![feature(iter_advance_by)]

mod ast;
mod current_iterator;
mod ir;
mod lexer;
mod parser;
mod syntax_error;

use ir::get_ir;
use parser::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(clap::Parser)]
struct Args {
    input_file: PathBuf,
    #[arg(short, default_value = "a.out")]
    output_file: PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    // Read input file into String
    let input_data = read_to_string(args.input_file).unwrap();

    let mut p = Parser::new(&input_data);
    let program = p.get_ast().unwrap().unwrap();
    let ir = get_ir(&program).unwrap();

    for stmt in ir {
        println!("{}", stmt);
    }
}
