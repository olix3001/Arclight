use inkwell::builder::Builder;

use crate::lexer::lexer::{Token, TokenType};

pub trait ASTExpr {
    fn generate(&self, builder: &mut Builder) -> ();
    fn to_string(&self) -> String;
}

pub trait Parseable {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String>;
}

#[derive(Debug)]
pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F8,
    F16,
    F32,
    F64,
    UF8,
    UF16,
    UF32,
    UF64,
    Bool,
    String,
    Void,
    Custom(String),
    Unknown
}

pub struct VoidExpr {}
impl ASTExpr for VoidExpr {
    fn generate(&self, _builder: &mut Builder) -> () {
        return;
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

impl DataType {
    pub fn parse(Token: &Token) -> Result<DataType, String> {
        match Token.token_type {
            TokenType::Identifier(ref s) => {
                match s.as_str() {
                    "i8" => Ok(DataType::I8),
                    "i16" => Ok(DataType::I16),
                    "i32" => Ok(DataType::I32),
                    "i64" => Ok(DataType::I64),
                    "u8" => Ok(DataType::U8),
                    "u16" => Ok(DataType::U16),
                    "u32" => Ok(DataType::U32),
                    "u64" => Ok(DataType::U64),
                    "f8" => Ok(DataType::F8),
                    "f16" => Ok(DataType::F16),
                    "f32" => Ok(DataType::F32),
                    "f64" => Ok(DataType::F64),
                    "uf8" => Ok(DataType::UF8),
                    "uf16" => Ok(DataType::UF16),
                    "uf32" => Ok(DataType::UF32),
                    "uf64" => Ok(DataType::UF64),
                    "bool" => Ok(DataType::Bool),
                    "string" => Ok(DataType::String),
                    "void" => Ok(DataType::Void),
                    _ => Ok(DataType::Custom(s.clone())),
                }
            }
            _ => Err(format!("Expected type, found {:?}", Token)),
        }
    }
}