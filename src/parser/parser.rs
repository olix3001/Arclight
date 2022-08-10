use crate::{lexer::lexer::Token, try_parse, utils::error::Error};

use super::expressions::{ASTExpr, import_expression::ImportExpr, Parseable, function_expression::FunctionExpr};


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
            // Global things to parse
            match try_parse!(self.tokens, self.pos, ImportExpr FunctionExpr) {
                Ok(expr) => { 
                    ast.push(expr);
                },
                Err(err) => {
                    err.print_err();
                    break;
                }
            }
        }
        
        ast
    }
    
    fn parse_expr<T>(&mut self) -> Result<Box<dyn ASTExpr>, Error> where T: Parseable {
        T::parse(self.tokens, &mut self.pos)
    }
    
}
