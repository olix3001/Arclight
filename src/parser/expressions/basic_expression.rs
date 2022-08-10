
use crate::{try_parse, lexer::lexer::TokenType, utils::{error::Error, error_components::token_component::ErrorTokenComponent}, error};

use super::{Parseable, block_expression::BlockExpr, value_expression::ValueExpr };

// Parses any basic expression
pub struct BasicExpr {}

impl Parseable for BasicExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn super::ASTExpr>, Error> {
        let temp = try_parse!(tokens, *pos, BlockExpr);
        if temp.is_ok() {
            *pos += 1;
            return temp;
        }
        let temp = try_parse!(tokens, *pos, ValueExpr);
        // Should be followed by a semicolon
        if tokens[*pos].token_type != TokenType::Separator(';') {
            return Err(error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing basic expression",
                              ErrorTokenComponent::new("Expected ';'".to_string(), Some(tokens[*pos].clone()))));
        }
        *pos += 1;
        return temp;
    }
}
