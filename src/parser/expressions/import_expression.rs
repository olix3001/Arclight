use inkwell::builder::Builder;

use crate::{lexer::lexer::{Token, TokenType}, parser_error};

use super::{ASTExpr, Parseable};

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
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        let mut path: Vec<String> = Vec::new();
        let mut imports: Vec<String> = Vec::new();
        
        // Should start with keyword "import"
        if tokens[*pos].token_type != TokenType::Identifier("import".to_string()) {
            return Err(format!("Expected import, found {:?}", tokens[*pos]));
        }
    
        // Should be followed by a path
        *pos += 1;
        while tokens[*pos].token_type != TokenType::Separator(';') && tokens[*pos].token_type != TokenType::Brace('{') {
            // Path
            match tokens[*pos].token_type {
                TokenType::Identifier(ref s) => path.push(s.clone()),
                _ => parser_error!(format!("Expected path, found {:?}", tokens[*pos]))
            }
            *pos += 1;
    
            if tokens[*pos].token_type == TokenType::Separator(';') {
                // Expect semicolon
                if tokens[*pos].token_type != TokenType::Separator(';') {
                    parser_error!(format!("Expected semicolon, found {:?}", tokens[*pos]));
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
                parser_error!(tokens[*pos], "Expected double colon, found only one");
            }
        }
        
        
        // Could be followed by imports
        if tokens[*pos].token_type == TokenType::Brace('{') {
            *pos += 1;
            while tokens[*pos].token_type != TokenType::Brace('}') {
                match tokens[*pos].token_type {
                    TokenType::Identifier(ref s) => imports.push(s.clone()),
                    _ => parser_error!(tokens[*pos], format!("Expected import, found {:?}", tokens[*pos])),
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
            parser_error!(format!("Expected semicolon, found {:?}", tokens[*pos]));
        }
        *pos += 1;
    
        Ok(Box::new(ImportExpr::new(path, imports)))
    }
}

impl ASTExpr for ImportExpr {
    fn generate(&self, builder: &mut Builder) -> () {
        todo!()
    }
    
    fn to_string(&self) -> String {
        return format!("Import {} {{ {} }}", self.path.join("::"), self.imports.join(", ")); 
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