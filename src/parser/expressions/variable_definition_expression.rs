use std::{rc::Rc};

use inkwell::{builder, values::{AnyValueEnum}};

use crate::{lexer::lexer::TokenType, parser::expressions::value_expression::ValueExpr, utils::{error::Error, error_components::token_component::ErrorTokenComponent}, error};

use super::{ASTExpr, Parseable, VoidExpr, data_types::{DataType, ToBasic}, scope::ScopeManager};

pub struct VarDefExpr {
    name: String,
    data_type: DataType,
    is_defined: bool,
    value: Box<dyn ASTExpr>,
    is_mutable: bool,
}

impl Parseable for VarDefExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, Error> {
        // Should start with keyword "var"
        if tokens[*pos].token_type != TokenType::Identifier("var".to_string()) {
            return Err(error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing variable declaration",
                              ErrorTokenComponent::new("Expected 'var' keyword".to_string(), Some(tokens[*pos].clone()))));
        }

        // Should be followed by a name
        *pos += 1;
        let mut name = "".to_string();
        match tokens[*pos].token_type {
            TokenType::Identifier(ref s) => {
                name = s.clone();
            }
            _ => error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing variable declaration",
                        ErrorTokenComponent::new("Expected variable name".to_string(), Some(tokens[*pos].clone()))).panic()
        }

        // TODO: Make type optional if can resolve type from value
        // Should be followed by a colon
        *pos += 1;
        if tokens[*pos].token_type != TokenType::Separator(':') {
            error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing variable type",
                   ErrorTokenComponent::new("Expected ':'".to_string(), Some(tokens[*pos].clone()))).panic()
        }

        // Should be followed by a type
        *pos += 1;
        let mut var_type = Ok(DataType::Unknown);
        let type_error = error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing variable type",
                                ErrorTokenComponent::new("Expected type".to_string(), Some(tokens[*pos].clone())));
        match tokens[*pos].token_type {
            TokenType::Identifier(ref s) => {
                var_type = DataType::parse(&tokens[*pos]);
                if !var_type.is_ok() {
                    type_error.panic()
                }
            }
            _ => type_error.panic()
        }
        if !var_type.is_ok() {
            type_error.panic()
        }

        // Can be followed by an equals sign
        *pos += 1;
        if tokens[*pos].token_type == TokenType::Operator("=".to_string()) {
            *pos += 1;
        } else {
            return Ok(Box::new(VarDefExpr {
                name,
                data_type: var_type.unwrap_or_else(|_| panic!("TODO")),
                is_defined: false,
                value: Box::new(VoidExpr {}),
                is_mutable: true,
            }));
        }
        
        // Should be followed by a value
        let value = ValueExpr::parse(tokens, pos);
        match value {
            Ok(v) => {
                Ok(Box::new(VarDefExpr {
                    name,
                    data_type: var_type.unwrap_or_else(|_| panic!("TODO")),
                    is_defined: true,
                    value: v,
                    is_mutable: true,
                }))
            }
            Err(_e) => {
                error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing variable definition",
                             ErrorTokenComponent::new("Expected variable value".to_string(), Some(tokens[*pos].clone()))).panic_val();
                panic!("Error while parsing variable definition")
            }
        }
    }
}

impl ASTExpr for VarDefExpr {
    fn to_string(&self) -> String {
        if self.is_defined {
            format!("Var {:?} {} = {}", self.data_type, self.name, self.value.to_string())
        } else {
            format!("Var {:?} {}", self.data_type, self.name)
        }
    }

    fn generate<'a, 'b>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &builder::Builder<'a>, scope_manager: &'b mut ScopeManager<'a>) -> Option<inkwell::values::AnyValueEnum<'a>> {
        if self.is_mutable {
            // Create alloca
            let alloca = builder.build_alloca(self.data_type.into_basic_type(context), &self.name);
            // Store value if defined
            if self.is_defined {
                let value = self.value.generate(context, module, builder, scope_manager);
                // TODO: Custom error if value is null
                builder.build_store(alloca, value.unwrap().to_basic());
            }
            // Add alloca to variables
            scope_manager.scope.variables.insert(self.name.to_string(), Rc::new(alloca));
            // Return alloca 
            Some(AnyValueEnum::PointerValue(alloca))
        } else {
            // Return value if defined
            if self.is_defined {
                self.value.generate(context, module, builder, scope_manager)
            } else {
                panic!("Variable '{}' is immutable and is not defined", self.name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::TokenType, test_token, parser::expressions::Parseable};


    #[test]
    fn parse_variable_without_value() {
        let tokens = vec![
            test_token!(TokenType::Identifier("var".to_string())),
            test_token!(TokenType::Identifier("x".to_string())),
            test_token!(TokenType::Separator(':')),
            test_token!(TokenType::Identifier("i32".to_string())),
            test_token!(TokenType::Separator(';')),
        ];
        let mut pos = 0;
        let expr = super::VarDefExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "Var I32 x");
    }

}
