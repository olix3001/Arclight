use std::collections::HashMap;

use inkwell::values::{PointerValue, FunctionValue};
use inkwell::basic_block::BasicBlock;

#[derive(Debug, Clone)]
pub struct Scope<'a, 'b> {
    pub variables: HashMap<String, PointerValue<'a>>,
    pub functions: HashMap<String, FunctionValue<'b>>,
}

#[derive(Debug)]
pub struct ScopeManager<'a, 'b> {
    pub scope: Scope<'a, 'b>,
    scopes: Vec<Box<Scope<'a, 'b>>>,
    pub function: Option<&'a FunctionValue<'a>>,
    pub block: Option<&'a BasicBlock<'a>>,
}

impl<'a, 'b> ScopeManager<'a, 'b> {
    pub fn new() -> ScopeManager<'a, 'b> {
        let mut global = Scope::new();
        ScopeManager {
            scope: global,
            scopes: vec![],
            function: None,
            block: None,
        }
    }

    pub fn create_scope(&mut self) {
        self.scopes.push(Box::new(self.scope.extend()));
        self.scope = Scope::new();
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() <= 0 { panic!("Cannot exit from the global scope") }
        self.scope = *self.scopes.pop().unwrap();
    }
}

impl<'a, 'b> Scope<'a, 'b> {
    pub fn new() -> Scope<'a, 'b> {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn extend(&self) -> Scope<'a, 'b> {
        Scope {
            variables: self.variables.clone(),
            functions: self.functions.clone(),
        }
    }
}
