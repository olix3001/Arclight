use inkwell::{builder::Builder, module::Module, context::Context, values::AnyValueEnum, basic_block::BasicBlock };

use crate::lexer::lexer::{Token, TokenType};

use self::function_expression::FunctionExpr;

pub trait ASTExpr {
    fn generate<'a>(&self, context: &'a Context, module: &Module<'a>, builder: &Builder<'a>, scope: Option<&Scope>) -> Option<inkwell::values::AnyValueEnum<'a>>;
    fn to_string(&self) -> String;
}

pub trait Parseable {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String>;
}


pub struct VoidExpr {}
impl ASTExpr for VoidExpr {
    fn generate<'a>(&self, context: &'a Context, module: &Module<'a>, builder: &Builder<'a>, scope: Option<&Scope>) -> Option<inkwell::values::AnyValueEnum<'a>> {
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

pub struct Scope<'a> {
    variables: Vec<(String, inkwell::values::AnyValueEnum<'a>)>,
    functions: Vec<(String, inkwell::values::FunctionValue<'a>)>,
    parent: Option<&'a Scope<'a>>,
    function: Option<&'a inkwell::values::FunctionValue<'a>>,
    block: Option<&'a BasicBlock<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        return Scope {
            variables: Vec::new(),
            functions: Vec::new(),
            parent: None,
            function: None,
            block: None,
        };
    }

    pub fn from_scope(scope: &'a Scope<'a>, function: Option<&'a inkwell::values::FunctionValue<'a>>, block: Option<&'a BasicBlock>) -> Scope<'a> {
        return Scope {
            variables: scope.variables.clone(),
            functions: scope.functions.clone(),
            parent: Some(scope),
            function: function,
            block: block,
        };
    }

    pub fn dispose<'b>(&self) -> &Scope {
        if self.parent.is_some() {
            // Return parent scope
            return self.parent.unwrap()
        } else {
            panic!("Attempted to dispose scope without parent");
        }
    }

    pub fn get_current_function(&self) -> Option<&inkwell::values::FunctionValue> {
        return self.function;
    }

    pub fn get_current_block(&self) -> Option<&BasicBlock> {
        return self.block;
    }

    pub fn add_variable(&mut self, name: String, value: inkwell::values::AnyValueEnum<'a>) {
        self.variables.push((name, value));
    }

    pub fn add_function(&mut self, name: String, value: inkwell::values::FunctionValue<'a>) {
        self.functions.push((name, value));
    }

    pub fn get_variable(&self, name: &str) -> Option<&inkwell::values::AnyValueEnum<'a>> {
        for (n, v) in &self.variables {
            if n == name {
                return Some(v);
            }
        }
        return None;
    }

    pub fn get_function(&self, name: &str) -> Option<&inkwell::values::FunctionValue<'a>> {
        for (n, v) in &self.functions {
            if n == name {
                return Some(v);
            }
        }
        return None;
    }

    pub fn get_variable_mut(&mut self, name: &str) -> Option<&mut inkwell::values::AnyValueEnum<'a>> {
        for (n, v) in &mut self.variables {
            if n == name {
                return Some(v);
            }
        }
        return None;
    }

    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut inkwell::values::FunctionValue<'a>> {
        for (n, v) in &mut self.functions {
            if n == name {
                return Some(v);
            }
        }
        return None;
    }
}