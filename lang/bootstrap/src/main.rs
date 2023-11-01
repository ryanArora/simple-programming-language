use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    input_file: PathBuf,
    #[arg(short)]
    output_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("input_file: {}", args.input_file.display());
    println!("output_file: {}", args.output_file.display());
}
