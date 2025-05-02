mod arch;
mod ast;
mod current_iterator;
mod interp;
mod ir;
mod lexer;
mod parser;
mod syntax_error;

use arch::Arch;
use parser::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;
use syntax_error::SyntaxError;

#[derive(clap::Parser)]
struct Args {
    input_file: PathBuf,
    #[arg(short, default_value = "a.out")]
    output_file: PathBuf,
    #[arg(long, value_enum, default_value_t=Arch::X86_64)]
    arch: Arch,
}

fn main() -> Result<(), SyntaxError> {
    let args = <Args as clap::Parser>::parse();

    // Read input file into String
    let input_data = read_to_string(args.input_file).unwrap();

    let mut p = Parser::new(&input_data);
    let program = p.get_ast()?.unwrap();
    let ir = ir::get_ir(&program)?;
    interp::interpret(&ir);

    Ok(())
}
