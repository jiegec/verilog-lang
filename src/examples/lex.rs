use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use verilog_lang::lexer::Lexer;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(args.file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let result = Lexer::lex(&content);
    println!("{:?}", result.tokens);
    println!("{:?}", result.diag);
}
