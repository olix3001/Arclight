use crate::{lexer::lexer::{TokenType, Token}, try_parse};

use super::{ASTExpr, Parseable, DataType};

pub struct LiteralExpr {}
impl Parseable for LiteralExpr {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        try_parse!(tokens, *pos, IntegerLiteralExpr)
    }
}


pub struct IntegerLiteralExpr {
    value: i64,
    dt: DataType
}

impl Parseable for IntegerLiteralExpr {
    fn parse(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        match &tokens[*pos].token_type {
            TokenType::Number(value) => {
                *pos += 1;
                return Ok(Box::new(IntegerLiteralExpr {
                    value: value.parse::<i64>().unwrap(),
                    dt: DataType::I32
                }));
            }
            _ => return Err(format!("Expected integer literal, found {:?}", tokens[*pos]))
        }
    }
}

impl ASTExpr for IntegerLiteralExpr {
    fn generate(&self, builder: &mut inkwell::builder::Builder) -> () {
        todo!()
    }

    fn to_string(&self) -> String {
        format!("{:?} {}", self.dt, self.value)
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
        assert_eq!(expr.to_string(), "I32 123");
    }

}