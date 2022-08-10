use std::{fs::File, io::Read};

use compiler::compiler::Compiler;
use inkwell::context::Context;

mod lexer;
mod parser;
mod compiler;
mod utils;

fn main() {
    let mut f = File::open("test.arl").unwrap();

    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let tokens = lexer::lexer::tokenize(content.as_str());

    for t in tokens.iter() {
        println!("{:?}", t);
    }

    let ast = parser::parser::parse(&tokens);

    for a in ast.iter() {
        println!("{}", a.to_string());
    }

    let context = Context::create();
    let module = context.create_module("test");
    let compiler = Compiler::new(&context, module);
    compiler.compile(ast);
}
