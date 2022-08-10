use inkwell::builder::Builder;

use crate::{lexer::lexer::{Token, TokenType}, utils::{error::Error, error_components::token_component::ErrorTokenComponent}, error};

use super::{ASTExpr, Parseable, scope::ScopeManager};

#[derive(Debug)]
pub struct ImportExpr {
    path: Vec<String>,
    imports: Vec<String>,
}

impl ImportExpr {
    fn new(path: Vec<String>, imports: Vec<String>) -> Self {
        Self {
            path,
            imports,
        }
    }
}

impl Parseable for ImportExpr {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, Error> {
        let mut path: Vec<String> = Vec::new();
        let mut imports: Vec<String> = Vec::new();
        
        // Should start with keyword "import"
        if tokens[*pos].token_type != TokenType::Identifier("import".to_string()) {
            return Err(error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing import expression",
                            ErrorTokenComponent::new("Expected 'import' keyword".to_string(), Some(tokens[*pos].clone()))));
        }
    
        // Should be followed by a path
        *pos += 1;
        while tokens[*pos].token_type != TokenType::Separator(';') && tokens[*pos].token_type != TokenType::Brace('{') {
            // Path
            match tokens[*pos].token_type {
                TokenType::Identifier(ref s) => path.push(s.clone()),
                _ => error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing import expression path",
                            ErrorTokenComponent::new("Expected path".to_string(), Some(tokens[*pos].clone()))).panic()
            }
            *pos += 1;
    
            if tokens[*pos].token_type == TokenType::Separator(';') {
                // Expect semicolon
                if tokens[*pos].token_type != TokenType::Separator(';') {
                    error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing import expression",
                           ErrorTokenComponent::new("Expected ';'".to_string(), Some(tokens[*pos].clone()))).panic()
                }
                *pos += 1;
                return Ok(Box::new(ImportExpr::new(path, imports)));
            }
    
            // Double colon
            if tokens[*pos].token_type == TokenType::Separator(':') {
                *pos += 1;
            } else {
                break;
            }
            if tokens[*pos].token_type == TokenType::Separator(':') {
                *pos += 1;
            } else {
                error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing import path",
                       ErrorTokenComponent::new("Expected '::'".to_string(), Some(tokens[*pos-1].clone()))).panic()
            }
        }
        
        
        // Could be followed by imports
        if tokens[*pos].token_type == TokenType::Brace('{') {
            *pos += 1;
            while tokens[*pos].token_type != TokenType::Brace('}') {
                match tokens[*pos].token_type {
                    TokenType::Identifier(ref s) => imports.push(s.clone()),
                    _ => error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing import names",
                                ErrorTokenComponent::new("Expected import name".to_string(), Some(tokens[*pos].clone()))).panic()
                }
                *pos += 1;
                if tokens[*pos].token_type == TokenType::Separator(',') {
                    *pos += 1;
                } else {
                    *pos += 1;
                    break;
                }
            }
        }
    
        // Should be followed by a semicolon
        if tokens[*pos].token_type != TokenType::Separator(';') {
            error!(crate::utils::error::ErrorKind::ParserError, "error while parsing import expression",
                   ErrorTokenComponent::new("Expected ';'".to_string(), Some(tokens[*pos].clone()))).panic()
        }
        *pos += 1;
    
        Ok(Box::new(ImportExpr::new(path, imports)))
    }
}

impl ASTExpr for ImportExpr {    
    fn to_string(&self) -> String {
        return format!("Import {} {{ {} }}", self.path.join("::"), self.imports.join(", ")); 
    }

    fn generate<'a, 'b>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &Builder, scope_manager: &'b mut ScopeManager<'a>) -> Option<inkwell::values::AnyValueEnum<'a>> {
        todo!()
    }
    
}


#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::{Token, TokenType}, parser::expressions::Parseable, test_token};


    #[test]
    fn parse_single_import_expression() {
        let tokens = vec![
            test_token!(TokenType::Identifier("import".to_string())), 
            test_token!(TokenType::Identifier("std".to_string())), 
            test_token!(TokenType::Separator(';')),
        ];
        let mut pos = 0;
        let expr = super::ImportExpr::parse(&tokens, &mut pos).unwrap();
        assert_eq!(expr.to_string(), "Import std {  }");
    }

    #[test]
    fn parse_multiple_import_expression() {
        let tokens = vec![
            test_token!(TokenType::Identifier("import".to_string())),
            test_token!(TokenType::Identifier("std".to_string())), 
            test_token!(TokenType::Separator(':')),
            test_token!(TokenType::Separator(':')),
            test_token!(TokenType::Identifier("test".to_string())),
            test_token!(TokenType::Separator(';')),
        ];
        let mut pos = 0;
        let expr = super::ImportExpr::parse(&tokens, &mut pos).unwrap();
        assert_eq!(expr.to_string(), "Import std::test {  }");
    }

    #[test]
    fn parse_select_import_expression() {
        let tokens = vec![
            test_token!(TokenType::Identifier("import".to_string())),
            test_token!(TokenType::Identifier("std".to_string())),
            test_token!(TokenType::Separator(':')),
            test_token!(TokenType::Separator(':')),
            test_token!(TokenType::Identifier("test".to_string())),
            test_token!(TokenType::Brace('{')),
            test_token!(TokenType::Identifier("hello".to_string())),
            test_token!(TokenType::Separator(',')),
            test_token!(TokenType::Identifier("world".to_string())),
            test_token!(TokenType::Brace('}')),
            test_token!(TokenType::Separator(';')),
        ];
        let mut pos = 0;
        let expr = super::ImportExpr::parse(&tokens, &mut pos).unwrap();
        assert_eq!(expr.to_string(), "Import std::test { hello, world }");
    }

}
