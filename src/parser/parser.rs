use crate::{lexer::lexer::Token, try_parse};

use super::expressions::{ASTExpr, import_expression::ImportExpr, Parseable};


pub fn parse(tokens: &Vec<Token>) -> Vec<Box<dyn ASTExpr>> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0
        }
    }

    fn parse(&mut self) -> Vec<Box<dyn ASTExpr>> {
        let mut ast: Vec<Box<dyn ASTExpr>> = Vec::new();

        while self.pos < self.tokens.len() {
            match try_parse!(self.tokens, self.pos, ImportExpr) {
                Ok(expr) => { 
                    ast.push(expr);
                },
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            }
        }
        
        ast
    }
    
    // fn parse_expr<T>(&mut self) -> Result<Box<dyn ASTExpr>, String> where T: Parseable {
    //     T::parse(self.tokens, &mut self.pos)
    // }
    
}
