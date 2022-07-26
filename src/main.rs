use std::{fs::File, io::BufReader};

mod lexer;
mod parser;

fn main() {
    let f = File::open("test.arl").unwrap();
    let mut f = BufReader::new(f);

    let tokens = lexer::lexer::tokenize(&mut f);

    for t in tokens.iter() {
        println!("{:?}", t);
    }

    let ast = parser::parser::parse(&tokens);

    for a in ast.iter() {
        println!("{}", a.to_string());
    }
}
