use std::sync::Arc;

use inkwell::values::AnyValueEnum;

use crate::{lexer::lexer::{TokenType, Token}, try_parse, utils::{error::Error, error_components::token_component::ErrorTokenComponent}, error};

use super::{ASTExpr, Parseable, scope::ScopeManager};

pub struct LiteralExpr {}
impl Parseable for LiteralExpr {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, Error> {
        try_parse!(tokens, *pos, IntegerLiteralExpr)
    }
}

#[derive(Debug, Clone, Copy)]
enum NumberValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}

pub struct IntegerLiteralExpr {
    value: NumberValue,
}

impl Parseable for IntegerLiteralExpr {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, Error> {
        match &tokens[*pos].token_type {
            TokenType::Number(value) => {
                *pos += 1;

                if value.contains('f') || value.contains('u') || value.contains('i') {
                    let s: String;
                    let v: String;
                    if value.contains('f') { s = "f".to_owned() + value.split('f').last().unwrap(); v = value.split('f').next().unwrap().to_string(); }
                    else if value.contains('u') { s = "u".to_owned() + value.split('u').last().unwrap(); v = value.split('u').next().unwrap().to_string(); }
                    else if value.contains('i') { s = "i".to_owned() + value.split('i').last().unwrap(); v = value.split('i').next().unwrap().to_string(); }
                    else { s = "".to_string(); v = value.to_string(); }

                    return Ok(Box::new(IntegerLiteralExpr {
                        value: match s.as_str() {
                            "i8" => NumberValue::I8(v.parse::<i8>().unwrap()),
                            "i16" => NumberValue::I16(v.parse::<i16>().unwrap()),
                            "i32" => NumberValue::I32(v.parse::<i32>().unwrap()),
                            "i64" => NumberValue::I64(v.parse::<i64>().unwrap()),
                            "u8" => NumberValue::U8(v.parse::<u8>().unwrap()),
                            "u16" => NumberValue::U16(v.parse::<u16>().unwrap()),
                            "u32" => NumberValue::U32(v.parse::<u32>().unwrap()),
                            "u64" => NumberValue::U64(v.parse::<u64>().unwrap()),
                            "f32" => NumberValue::F32(v.parse::<f32>().unwrap()),
                            "f64" => NumberValue::F64(v.parse::<f64>().unwrap()),
                            _ => NumberValue::I8(0)
                        }
                    }));
                }

                if value.contains('.') {
                    return Ok(Box::new(IntegerLiteralExpr {
                        value: NumberValue::F64(value.parse::<f64>().unwrap()),
                    }));
                }

                return Ok(Box::new(IntegerLiteralExpr {
                    value: NumberValue::I32(value.parse::<i32>().unwrap()),
                }));
            }
            _ => return Err(error!(crate::utils::error::ErrorKind::ParserError, "Error while parsing integer literal",
                                   ErrorTokenComponent::new("Expected number literal".to_string(), Some(tokens[*pos].clone()))))
        }
    }
}

impl ASTExpr for IntegerLiteralExpr {
    fn to_string(&self) -> String {
        format!("{:?}", self.value)
    }

    fn generate<'a, 'b>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &inkwell::builder::Builder, scope_manager: &'b mut ScopeManager<'a>) -> Option<inkwell::values::AnyValueEnum<'a>> {
        match self.value {
            NumberValue::I8(value) => Some(AnyValueEnum::IntValue(context.i8_type().const_int(value as u64, true))),
            NumberValue::I16(value) => Some(AnyValueEnum::IntValue(context.i16_type().const_int(value as u64, true))),
            NumberValue::I32(value) => Some(AnyValueEnum::IntValue(context.i32_type().const_int(value as u64, true))),
            NumberValue::I64(value) => Some(AnyValueEnum::IntValue(context.i64_type().const_int(value as u64, true))),
            NumberValue::U8(value) => Some(AnyValueEnum::IntValue(context.i8_type().const_int(value as u64, false))),
            NumberValue::U16(value) => Some(AnyValueEnum::IntValue(context.i16_type().const_int(value as u64, false))),
            NumberValue::U32(value) => Some(AnyValueEnum::IntValue(context.i32_type().const_int(value as u64, false))),
            NumberValue::U64(value) => Some(AnyValueEnum::IntValue(context.i64_type().const_int(value as u64, false))),
            NumberValue::F32(value) => Some(AnyValueEnum::FloatValue(context.f32_type().const_float(value as f64))),
            NumberValue::F64(value) => Some(AnyValueEnum::FloatValue(context.f64_type().const_float(value))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::{TokenType}, parser::expressions::{literal_expression::IntegerLiteralExpr, Parseable}, test_token};

    #[test]
    fn parse_i32_number() {
        let tokens = vec![test_token!(TokenType::Number("123".to_string()))];
        let mut pos = 0;
        let expr = IntegerLiteralExpr::parse(&tokens, &mut pos).unwrap();
        assert_eq!(expr.to_string(), "I32(123)");
    }

    #[test]
    fn parse_f64_number() {
        let tokens = vec![test_token!(TokenType::Number("123.456".to_string()))];
        let mut pos = 0;
        let expr = IntegerLiteralExpr::parse(&tokens, &mut pos).unwrap();
        assert_eq!(expr.to_string(), "F64(123.456)");
    }

}
