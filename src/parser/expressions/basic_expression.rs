use inkwell::values::BasicValue;

use crate::{try_parse, lexer::lexer::TokenType};

use super::{Parseable, block_expression::BlockExpr, value_expression::ValueExpr };

// Parses any basic expression
pub struct BasicExpr {}

impl Parseable for BasicExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn super::ASTExpr>, String> {
        let temp = try_parse!(tokens, *pos, BlockExpr);
        if temp.is_ok() {
            *pos += 1;
            return temp;
        }
        let temp = try_parse!(tokens, *pos, ValueExpr);
        // Should be followed by a semicolon
        if tokens[*pos].token_type != TokenType::Separator(';') {
            return Err(format!("Expected ;, found {:?}", tokens[*pos]));
        }
        *pos += 1;
        return temp;
    }
}