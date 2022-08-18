use crate::{try_parse, utils::error::Error};

use super::{Parseable, math_expression::MathExpr, variable_definition_expression::VarDefExpr, block_expression::BlockExpr, literal_expression::LiteralExpr, variable_expression::VariableCallExpr }; 

pub struct ValueExpr {}
pub struct NoMathValueExpr {}

impl Parseable for ValueExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn super::ASTExpr>, Error> {
        return try_parse!(tokens, *pos, MathExpr NoMathValueExpr);
    }
}

impl Parseable for NoMathValueExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn super::ASTExpr>, Error> {
        return try_parse!(tokens, *pos, LiteralExpr BlockExpr VarDefExpr VariableCallExpr);
    }
}
