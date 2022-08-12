use std::collections::HashMap;
use std::rc::Rc;

use inkwell::values::{PointerValue, FunctionValue, BasicValueEnum};
use inkwell::basic_block::BasicBlock;

#[derive(Debug)]
pub struct Scope<'a> {
    pub variables: HashMap<String, Rc<PointerValue<'a>>>,
    pub functions: HashMap<String, Rc<FunctionValue<'a>>>,
    pub fn_args: HashMap<String, Rc<BasicValueEnum<'a>>>,
    pub function: Option<Rc<FunctionValue<'a>>>,
    pub block: Option<Rc<BasicBlock<'a>>>
}

#[derive(Debug)]
pub struct ScopeManager<'a> {
    pub scope: Scope<'a>,
    scopes: Vec<Box<Scope<'a>>>,
    pub function: Option<&'a FunctionValue<'a>>,
    pub block: Option<&'a BasicBlock<'a>>,
}

impl<'a> ScopeManager<'a> {
    pub fn new() -> ScopeManager<'a> {
        let mut global = Scope::new();
        ScopeManager {
            scope: global,
            scopes: vec![],
            function: None,
            block: None,
        }
    }

    pub fn create_scope(&mut self) {
        let scope = self.scope.extend();
        self.scopes.push(Box::new(self.scope.extend()));
        self.scope = scope; 
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() <= 0 { panic!("Cannot exit from the global scope") }
        self.scope = *self.scopes.pop().unwrap();
    }
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
            fn_args: HashMap::new(),
            function: None,
            block: None,
        }
    }

    pub fn extend(&self) -> Scope<'a> {
        Scope {
            variables: self.variables.clone(),
            functions: self.functions.clone(),
            fn_args: self.fn_args.clone(),
            function: None,
            block: None
        }
    }
}
