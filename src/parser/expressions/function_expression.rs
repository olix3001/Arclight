use std::collections::HashMap;

use inkwell::builder::Builder;

use crate::{lexer::lexer::TokenType, parser_error};

use super::{ASTExpr, DataType, Parseable, VoidExpr, basic_expression::BasicExpr};

pub struct FunctionExpr {
    body: Box<dyn ASTExpr>,
    arguments: HashMap<String, DataType>,
    return_type: DataType,
    name: String,
    // TODO: Add generics
}

impl Parseable for FunctionExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        let mut arguments: HashMap<String, DataType> = HashMap::new();
        let mut return_type: DataType = DataType::Void;
        let mut body: Box<dyn ASTExpr> = Box::new(VoidExpr {});
        let mut name: String = String::new();
        
        // Should start with keyword "fn"
        if tokens[*pos].token_type != TokenType::Identifier("fun".to_string()) {
            return Err(format!("Expected fun, found {:?}", tokens[*pos]));
        }

        // Should be followed by a name
        *pos += 1;
        match tokens[*pos].token_type {
            TokenType::Identifier(ref s) => { name = s.clone(); },
            _ => parser_error!(format!("Expected name, found {:?}", tokens[*pos]))
        }

        // TODO: Add generics

        // Should be followed by a parenthesis
        *pos += 1;
        if tokens[*pos].token_type != TokenType::Paren('(') {
            parser_error!(format!("Expected parenthesis, found {:?}", tokens[*pos]));
        }

        // Should be followed by a list of arguments
        *pos += 1;
        while tokens[*pos].token_type != TokenType::Paren(')') {
            // Argument name
            match tokens[*pos].token_type {
                TokenType::Identifier(ref s) => {
                    *pos += 1;
                    if tokens[*pos].token_type != TokenType::Separator(':') {
                        parser_error!(format!("Expected colon, found {:?}", tokens[*pos]));
                    }
                    *pos += 1;
                    let data_type = DataType::parse(&tokens[*pos])?;
                    *pos += 1;
                    arguments.insert(s.clone(), data_type);
                },
                _ => parser_error!(format!("Expected argument name, found {:?}", tokens[*pos]))
            }
            if tokens[*pos].token_type == TokenType::Separator(',') {
                *pos += 1;
            } else {
                break;
            }
        }

        // Should be followed by a colon
        *pos += 1;
        if tokens[*pos].token_type != TokenType::Separator(':') {
            parser_error!(format!("Expected colon, found {:?}", tokens[*pos]));
        }

        // Should be followed by a return type
        *pos += 1;
        return_type = DataType::parse(&tokens[*pos])?;

        // Should be followed by a function body
        *pos += 1;
        let t = BasicExpr::parse(tokens, pos);
        match t {
            Ok(expr) => body = expr,
            Err(_) => parser_error!(format!("Expected function body, found {:?}", tokens[*pos]))
        }

        // Return function
        Ok(Box::new(FunctionExpr {
            body,
            arguments,
            return_type,
            name,
        }))
    }
}

impl ASTExpr for FunctionExpr {
    fn generate(&self, builder: &mut Builder) -> () {
        todo!();
    }
    fn to_string(&self) -> String {
        format!("Function ({:?}) => {:?} {}", self.arguments, self.return_type, self.body.to_string())
    }
}