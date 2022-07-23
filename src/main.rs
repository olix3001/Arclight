use std::{fs::File, io::BufReader};

mod lexer;

fn main() {
    let f = File::open("spec/examples/hello_world.arl").unwrap();
    let mut f = BufReader::new(f);

    let tokens = lexer::lexer::tokenize(&mut f);

    for token in tokens {
        println!("{:?}", token);
    }
}
