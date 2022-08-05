use inkwell::{context::Context, module::Module};

use crate::parser::expressions::{ASTExpr, Scope};

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
        let global_scope = Scope::new();
        for expr in ast {
            expr.generate(self.context, &self.module, &builder, Some(&global_scope));
        }

        // Print generated llvm
        let llvm_str = self.module.print_to_string();
        println!("\n\nGenerated LLVM IR:\n{}", llvm_str.to_string());
    }
}