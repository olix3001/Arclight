use crate::try_parse;

use super::{Parseable, variable_definition_expression::VarDefExpr, block_expression::BlockExpr, literal_expression::LiteralExpr, variable_expression::VariableCallExpr }; 

pub struct ValueExpr {}

impl Parseable for ValueExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn super::ASTExpr>, String> {
        return try_parse!(tokens, *pos, LiteralExpr BlockExpr VarDefExpr VariableCallExpr);
    }
}