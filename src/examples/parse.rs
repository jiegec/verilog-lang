use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use verilog_lang::{ast::*, parser::Parser};

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long)]
    file: PathBuf,
}

#[paw::main]
fn main(args: Args) {
    let mut file = File::open(args.file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut parser = Parser::from(&content);
    let m = SourceText::parse(&mut parser);
    println!("{:?}", parser);
    println!("{:?}", m);
}
