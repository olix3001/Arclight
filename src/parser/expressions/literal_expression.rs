use crate::{lexer::lexer::{TokenType, Token}, try_parse};

use super::{ASTExpr, Parseable};

pub struct LiteralExpr {}
impl Parseable for LiteralExpr {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
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
    F32(f64),
    F64(f64),
}

pub struct IntegerLiteralExpr {
    value: NumberValue,
}

impl Parseable for IntegerLiteralExpr {
    // TODO: Support more types (refactor this code)
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        match &tokens[*pos].token_type {
            TokenType::Number(value) => {
                *pos += 1;

                if value.contains('.') {
                    return Ok(Box::new(IntegerLiteralExpr {
                        value: NumberValue::F64(value.parse::<f64>().unwrap()),
                    }));
                }

                return Ok(Box::new(IntegerLiteralExpr {
                    value: NumberValue::I32(value.parse::<i32>().unwrap()),
                }));
            }
            _ => return Err(format!("Expected integer literal, found {:?}", tokens[*pos]))
        }
    }
}

impl ASTExpr for IntegerLiteralExpr {
    fn to_string(&self) -> String {
        format!("{:?}", self.value)
    }

    fn generate<'a>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &inkwell::builder::Builder) -> Option<inkwell::values::AnyValueEnum<'a>> {
        todo!()
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