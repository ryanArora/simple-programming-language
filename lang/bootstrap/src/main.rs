#![feature(iter_advance_by)]
#![feature(let_chains)]

mod current_iterator;
mod lexer;

use lexer::Lexer;
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
    let mut l = Lexer::new(&input_data);
    let tokens = l.get_tokens().unwrap();
    println!("{:?}", tokens)
}
