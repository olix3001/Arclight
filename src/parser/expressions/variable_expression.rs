use inkwell::values::{AnyValueEnum, AnyValue};

use crate::{lexer::lexer::TokenType, utils::{error::Error, error_components::token_component::ErrorTokenComponent}, error};

use super::{Parseable, ASTExpr, data_types::ToAny, ScopeManager};

pub struct VariableCallExpr {
    name: String,
}

impl Parseable for VariableCallExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn super::ASTExpr>, Error> {
        let name = &tokens[*pos].token_type;
        match name {
            TokenType::Identifier(ref s) => {
                *pos += 1;
                return Ok(Box::new(VariableCallExpr {
                    name: s.clone(),
                }));
            },
            _ => return Err(error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing variable call",
                                   ErrorTokenComponent::new("Expected variable name".to_string(), Some(tokens[*pos].clone()))))
        }
    }
}

impl ASTExpr for VariableCallExpr {
    fn generate<'a, 'b>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &inkwell::builder::Builder<'a>, scope_manager: &'b mut ScopeManager<'a>) -> Option<inkwell::values::AnyValueEnum<'a>> {
        // Get function argument
        let fn_var = scope_manager.scope.fn_args.get(&self.name);
        if fn_var.is_some() {
            return Some(fn_var.unwrap().as_any_value_enum())
        }
        // Get normal variable
        let var = scope_manager.scope.variables.get(&self.name);
        if var.is_none() {
            return None;
        }
        let var = var.unwrap();
        let var = *var.clone();
        let load = builder.build_load(var, self.name.as_str());
        let nload = load.as_any_value_enum();
        Some(nload)
    }
 
    fn to_string(&self) -> String {
        format!("VarCall {}", self.name)
    }
}
