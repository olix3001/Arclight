use crate::{lexer::lexer::TokenType, parser_error, parser::expressions::value_expression::ValueExpr};

use super::{ASTExpr, Parseable, DataType, VoidExpr, basic_expression::BasicExpr};

pub struct VarDefExpr {
    name: String,
    data_type: DataType,
    is_defined: bool,
    value: Box<dyn ASTExpr>,
}

impl Parseable for VarDefExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, String> {
        // Should start with keyword "var"
        if tokens[*pos].token_type != TokenType::Identifier("var".to_string()) {
            return Err(format!("Expected var, found {:?}", tokens[*pos]));
        }

        // Should be followed by a name
        *pos += 1;
        let mut name = "".to_string();
        match tokens[*pos].token_type {
            TokenType::Identifier(ref s) => {
                name = s.clone();
            }
            _ => parser_error!(format!("Expected variable name, found {:?}", tokens[*pos]))
        }

        // TODO: Make type optional if can resolve type from value
        // Should be followed by a colon
        *pos += 1;
        if tokens[*pos].token_type != TokenType::Separator(':') {
            parser_error!(format!("Expected colon, found {:?}", tokens[*pos]));
        }

        // Should be followed by a type
        *pos += 1;
        let mut var_type = Ok(DataType::Unknown);
        match tokens[*pos].token_type {
            TokenType::Identifier(ref s) => {
                var_type = DataType::parse(&tokens[*pos]);
                if !var_type.is_ok() {
                    parser_error!(format!("Expected type, found {:?}", tokens[*pos]));
                }
            }
            _ => parser_error!(format!("Expected type, found {:?}", tokens[*pos]))
        }
        if !var_type.is_ok() {
            parser_error!(format!("Expected type, found {:?}", tokens[*pos]));
        }

        // Can be followed by an equals sign
        *pos += 1;
        if tokens[*pos].token_type == TokenType::Operator("=".to_string()) {
            *pos += 1;
        } else {
            return Ok(Box::new(VarDefExpr {
                name,
                data_type: var_type.unwrap(),
                is_defined: false,
                value: Box::new(VoidExpr {})
            }));
        }
        
        // Should be followed by a value
        let value = ValueExpr::parse(tokens, pos);
        match value {
            Ok(v) => {
                Ok(Box::new(VarDefExpr {
                    name,
                    data_type: var_type.unwrap(),
                    is_defined: true,
                    value: v
                }))
            }
            Err(e) => parser_error!(tokens[*pos], "Could not parse value")
        }
    }
}

impl ASTExpr for VarDefExpr {
    fn generate(&self, builder: &mut inkwell::builder::Builder) -> () {
        todo!()
    }

    fn to_string(&self) -> String {
        if self.is_defined {
            format!("Var {:?} {} = {}", self.data_type, self.name, self.value.to_string())
        } else {
            format!("Var {:?} {}", self.data_type, self.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::TokenType, test_token, parser::expressions::Parseable};


    #[test]
    fn parse_variable_without_value() {
        let tokens = vec![
            test_token!(TokenType::Identifier("var".to_string())),
            test_token!(TokenType::Identifier("x".to_string())),
            test_token!(TokenType::Separator(':')),
            test_token!(TokenType::Identifier("i32".to_string())),
            test_token!(TokenType::Separator(';')),
        ];
        let mut pos = 0;
        let expr = super::VarDefExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "Var I32 x");
    }

}