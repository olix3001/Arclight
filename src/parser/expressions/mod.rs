use inkwell::{builder::Builder, module::Module, context::Context };

use crate::lexer::lexer::{Token, TokenType};

pub trait ASTExpr {
    fn generate<'a>(&self, context: &'a Context, module: &Module<'a>, builder: &Builder) -> Option<inkwell::values::AnyValueEnum<'a>>;
    fn to_string(&self) -> String;
}

pub trait Parseable {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String>;
}


pub struct VoidExpr {}
impl ASTExpr for VoidExpr {
    fn generate<'a>(&self, context: &'a Context, module: &Module<'a>, builder: &Builder) -> Option<inkwell::values::AnyValueEnum<'a>> {
        return None;
    }
    fn to_string(&self) -> String {
        "NOP".to_string()
    }
}
impl Parseable for VoidExpr {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        return Err("Void expression is not parseable".to_string());
    }
}

pub mod import_expression;
pub mod function_expression;
pub mod block_expression;
pub mod basic_expression;
pub mod variable_definition_expression;
pub mod value_expression;
pub mod literal_expression;
pub mod data_types;

