use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use verilog_lang::ast::*;
use verilog_lang::parser::Parser as VerilogParser;

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
    let mut parser = VerilogParser::from(&content);
    let m = SourceText::parse(&mut parser);
    println!("{:?}", parser);
    println!("{:?}", m);
}
