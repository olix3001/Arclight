use inkwell::builder::Builder;

use crate::lexer::lexer::Token;

pub trait ASTExpr {
    fn generate(&self, builder: &mut Builder) -> ();
    fn to_string(&self) -> String;
}

pub trait Parseable {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String>;
}

pub mod import_expression;
