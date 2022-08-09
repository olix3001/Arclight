use inkwell::{context::Context, module::Module};

use crate::parser::expressions::{ASTExpr, scope::ScopeManager};

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Compiler<'ctx> {
        Compiler {
            context: &context,
            module,
        }
    }

    pub fn compile(&self, ast: Vec<Box<dyn ASTExpr>>) -> () {

        // Create a builder
        let builder = self.context.create_builder();

        // Go through AST and compile each expression
        let mut sm = ScopeManager::new();
        for expr in ast {
            expr.generate(self.context, &self.module, &builder, &mut sm); 
        }

        // Print generated llvm
        let llvm_str = self.module.print_to_string();
        println!("\n\nGenerated LLVM IR:\n{}", llvm_str.to_string());
    }
}