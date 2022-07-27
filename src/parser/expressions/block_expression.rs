use crate::lexer::lexer::TokenType;

use super::{Parseable, ASTExpr, basic_expression::BasicExpr};

pub struct BlockExpr {
    statements: Vec<Box<dyn ASTExpr>>,
}

impl Parseable for BlockExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        // Should start with brace
        if tokens[*pos].token_type != TokenType::Brace('{') {
            return Err(format!("Expected {{, found {:?}", tokens[*pos]));
        }

        *pos += 1;
        let mut statements: Vec<Box<dyn ASTExpr>> = Vec::new();
        while tokens[*pos].token_type != TokenType::Brace('}') {
            let expr = BasicExpr::parse(tokens, pos)?;
            statements.push(expr);
        }

        *pos += 1;
        Ok(Box::new(BlockExpr {
            statements,
        }))
    }
}

impl ASTExpr for BlockExpr {
    fn generate(&self, builder: &mut inkwell::builder::Builder) -> () {
        todo!()
    }

    fn to_string(&self) -> String {
        format!("{{\n {} \n}}", self.statements.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n"))
    }
}