#![feature(iter_advance_by)]

mod ast;
mod current_iterator;
mod lexer;
mod parser;
mod syntax_error;

use parser::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(clap::Parser)]
struct Args {
    input_file: PathBuf,
    #[arg(short)]
    output_file: PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    println!("input_file: {}", args.input_file.display());
    println!("output_file: {}", args.output_file.display());

    // Read input file into String
    let input_data = read_to_string(args.input_file).unwrap();

    let mut p = Parser::new(&input_data);
    let ast = p.get_ast();

    println!("{:?}", ast);
}
