

use inkwell::values::{AnyValueEnum, AnyValue};

use crate::lexer::lexer::TokenType;

use super::{Parseable, ASTExpr, data_types::ToAny, ScopeManager};

pub struct VariableCallExpr {
    name: String,
}

impl Parseable for VariableCallExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn super::ASTExpr>, String> {
        let name = &tokens[*pos].token_type;
        match name {
            TokenType::Identifier(ref s) => {
                *pos += 1;
                return Ok(Box::new(VariableCallExpr {
                    name: s.clone(),
                }));
            },
            _ => return Err(format!("Expected identifier, found {:?}", tokens[*pos]))
        }
    }
}

impl ASTExpr for VariableCallExpr {
    fn generate<'a, 'b>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &inkwell::builder::Builder<'a>, scope_manager: &'b mut ScopeManager<'a>) -> Option<inkwell::values::AnyValueEnum<'a>> {
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
