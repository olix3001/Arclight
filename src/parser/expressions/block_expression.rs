use crate::{lexer::lexer::TokenType, utils::{error::Error, error_components::token_component::ErrorTokenComponent}, error};

use super::{Parseable, ASTExpr, basic_expression::BasicExpr, function_expression::FunctionExpr, scope::ScopeManager};

pub struct BlockExpr {
    statements: Vec<Box<dyn ASTExpr>>,
}

impl Parseable for BlockExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, Error> {
        // Should start with brace
        if tokens[*pos].token_type != TokenType::Brace('{') {
            return Err(error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing block expression",
                              ErrorTokenComponent::new("Expected '{{'".to_string(), Some(tokens[*pos].clone()))));
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
        format!("{{\n\t {} \n}}", self.statements.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n\t "))
    }

    fn generate<'a, 'b>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &inkwell::builder::Builder<'a>, scope_manager: &'b mut ScopeManager<'a>) -> Option<inkwell::values::AnyValueEnum<'a>> {
        // let function = sm.function.as_ref().unwrap();
        // let block_block = context.append_basic_block(**function, "code_block");
        // builder.build_unconditional_branch(block_block );
        // builder.position_at_end(block_block);

        scope_manager.create_scope();
        for statement in &self.statements {
            statement.generate(context, module, builder, scope_manager);
        }
        scope_manager.exit_scope();

        // let after_block = context.append_basic_block(**function, "after_block");
        // builder.build_unconditional_branch(after_block);
        // builder.position_at_end(after_block);
        return None;
    }

    
}
