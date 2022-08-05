use crate::lexer::lexer::TokenType;

use super::{Parseable, ASTExpr, basic_expression::BasicExpr, function_expression::FunctionExpr, Scope};

pub struct BlockExpr {
    statements: Vec<Box<dyn ASTExpr>>,
}

impl Parseable for BlockExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        // Should start with brace
        if tokens[*pos].token_type != TokenType::Brace('{') {
            return Err(format!("Expected {{, found {:?}", tokens[*pos]));
        }

        *pos += 1;
        let mut statements: Vec<Box<dyn ASTExpr>> = Vec::new();
        while tokens[*pos].token_type != TokenType::Brace('}') {
            let expr = BasicExpr::parse(tokens, pos)?;
            statements.push(expr);
        }

        *pos += 1;
        Ok(Box::new(BlockExpr {
            statements,
        }))
    }
}

impl ASTExpr for BlockExpr {

    fn to_string(&self) -> String {
        format!("{{\n {} \n}}", self.statements.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n"))
    }

    fn generate<'a>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &inkwell::builder::Builder<'a>, scope: Option<&Scope>) -> Option<inkwell::values::AnyValueEnum<'a>> {
        let block_block = context.append_basic_block(*scope.unwrap().get_current_function().unwrap(), "code_block");
        builder.build_unconditional_branch(block_block);
        builder.position_at_end(block_block);

        for statement in &self.statements {
            statement.generate(context, module, builder, scope);
        }

        let after_block = context.append_basic_block(*scope.unwrap().get_current_function().unwrap(), "after_block");
        builder.build_unconditional_branch(after_block);
        builder.position_at_end(after_block);
        return None;
    }

    
}