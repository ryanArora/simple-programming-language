mod analyze;
mod lexer;
mod parser;

use analyze::analyze;
use lexer::Lexer;
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

    // Parse text input into tokens
    let l = Lexer::new(input_data.chars());
    let tokens = l.get_tokens().unwrap();

    // Parse tokens into an AST
    let p = Parser::new(tokens);
    let ast = p.get_ast().unwrap();

    // Semantic Analysis
    analyze(ast).unwrap();
}
