
use inkwell::values::{AnyValueEnum, IntValue};

use crate::{error, utils::{error::{Error, ErrorKind}, error_components::name_component::NameErrorComponent}, lexer::lexer::TokenType};

use super::{ASTExpr, Parseable, value_expression};

#[derive(Debug)]
enum MathOperation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE
}

pub struct MathExpr {
    lhs: Box<dyn ASTExpr>,
    rhs: Box<dyn ASTExpr>,
    operation: MathOperation,
}

impl Parseable for MathExpr {
    fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn ASTExpr>, crate::utils::error::Error> {
        // Get LHS expression
        let lhs = value_expression::NoMathValueExpr::parse(tokens, pos);
        if lhs.is_err() {
            return Err(error!(ErrorKind::ParserError, "Expected LHS expression"));
        }
        // Get operation
        let op = match &tokens[*pos].token_type {
            TokenType::Operator(ref op) => {
                match op.chars().next().unwrap() {
                    '+' => MathOperation::ADD,
                    '-' => MathOperation::SUBTRACT,
                    '*' => MathOperation::MULTIPLY,
                    '/' => MathOperation::DIVIDE,
                    _ => { return Err(error!(ErrorKind::ParserError, "Expected math operator")) }
                }
            }
            _ => { return Err(error!(ErrorKind::ParserError, "Expected math operator")) } 
        };
        *pos += 1;

        // Get RHS expression
        let rhs = value_expression::ValueExpr::parse(tokens, pos);
        if rhs.is_err() {
            error!(ErrorKind::ParserError, "Expected RHS expression").panic();
        }

        Ok(Box::new(MathExpr {
            lhs: lhs.unwrap(),
            operation: op,
            rhs: rhs.unwrap()
        }))

    }
}


impl ASTExpr for MathExpr {
    fn generate<'a, 'b>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &inkwell::builder::Builder<'a>, scope_manager: &'b mut super::scope::ScopeManager<'a>) -> Option<inkwell::values::AnyValueEnum<'a>> {
        let lhs = self.lhs.generate(context, module, builder, scope_manager);
        if lhs.is_none() {
            error!(ErrorKind::CompilerError, "Expected LHS for binary expression").panic();
        }
        let rhs = self.rhs.generate(context, module, builder, scope_manager);
        if rhs.is_none() {
            error!(ErrorKind::CompilerError, "Expected RHS for binary expression").panic();
        }

        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();
        // TODO: Check if types are matching
        if lhs.get_type() != rhs.get_type() {
            error!(ErrorKind::CompilerError, "Binary operation types do not match",
                   NameErrorComponent::new(format!("LHS: {}", lhs.get_type()))
                   NameErrorComponent::new(format!("RHS: {}", lhs.get_type()))).panic();
        }
        if lhs.is_int_value() {
            return Some(AnyValueEnum::IntValue(match self.operation {
                MathOperation::ADD => builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "iaddtmp"),
                MathOperation::SUBTRACT => builder.build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "isubtmp"),
                MathOperation::MULTIPLY => builder.build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "imultmp"),
                MathOperation::DIVIDE => builder.build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "isdivtmp")
            }))
        } else if lhs.is_float_value() {
            return Some(AnyValueEnum::FloatValue(match self.operation {
                MathOperation::ADD => builder.build_float_add(lhs.into_float_value(), rhs.into_float_value(), "faddtmp"),
                MathOperation::SUBTRACT => builder.build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "fsubtmp"),
                MathOperation::MULTIPLY => builder.build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "fmultmp"),
                MathOperation::DIVIDE => builder.build_float_div(lhs.into_float_value(), rhs.into_float_value(), "fdivtmp")
            }))
        }

        todo!("Implement some error")
    }

    fn to_string(&self) -> String {
        format!("{:?} {}, {}", self.operation, self.lhs.to_string(), self.rhs.to_string())
    }
}


#[cfg(test)]
mod tests {
    use crate::{test_token, lexer::lexer::TokenType, parser::expressions::Parseable};

    #[test]
    fn parse_add_i32() {
        let tokens = vec![
            test_token!(TokenType::Number("5".to_string())),
            test_token!(TokenType::Operator("+".to_string())),
            test_token!(TokenType::Number("3".to_string())),
            test_token!(TokenType::Separator(';'))
        ];
        let mut pos = 0;
        let expr = super::MathExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "ADD I32(5), I32(3)");
    }

    #[test]
    fn parse_sub_i32() {
        let tokens = vec![
            test_token!(TokenType::Number("5".to_string())),
            test_token!(TokenType::Operator("-".to_string())),
            test_token!(TokenType::Number("3".to_string())),
            test_token!(TokenType::Separator(';'))
        ];
        let mut pos = 0;
        let expr = super::MathExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "SUBTRACT I32(5), I32(3)");
    }

    #[test]
    fn parse_mul_i32() {
        let tokens = vec![
            test_token!(TokenType::Number("5".to_string())),
            test_token!(TokenType::Operator("*".to_string())),
            test_token!(TokenType::Number("3".to_string())),
            test_token!(TokenType::Separator(';'))
        ];
        let mut pos = 0;
        let expr = super::MathExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "MULTIPLY I32(5), I32(3)");
    }

    #[test]
    fn parse_div_i32() {
        let tokens = vec![
            test_token!(TokenType::Number("5".to_string())),
            test_token!(TokenType::Operator("/".to_string())),
            test_token!(TokenType::Number("3".to_string())),
            test_token!(TokenType::Separator(';'))
        ];
        let mut pos = 0;
        let expr = super::MathExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "DIVIDE I32(5), I32(3)");
    }

    #[test]
    fn parse_add_f64() {
        let tokens = vec![
            test_token!(TokenType::Number("5.0".to_string())),
            test_token!(TokenType::Operator("+".to_string())),
            test_token!(TokenType::Number("3.0".to_string())),
            test_token!(TokenType::Separator(';'))
        ];
        let mut pos = 0;
        let expr = super::MathExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "ADD F64(5.0), F64(3.0)");
    }

    #[test]
    fn parse_sub_f64() {
        let tokens = vec![
            test_token!(TokenType::Number("5.0".to_string())),
            test_token!(TokenType::Operator("-".to_string())),
            test_token!(TokenType::Number("3.0".to_string())),
            test_token!(TokenType::Separator(';'))
        ];
        let mut pos = 0;
        let expr = super::MathExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "SUBTRACT F64(5.0), F64(3.0)");
    }

    #[test]
    fn parse_mul_f64() {
        let tokens = vec![
            test_token!(TokenType::Number("5.0".to_string())),
            test_token!(TokenType::Operator("*".to_string())),
            test_token!(TokenType::Number("3.0".to_string())),
            test_token!(TokenType::Separator(';'))
        ];
        let mut pos = 0;
        let expr = super::MathExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "MULTIPLY F64(5.0), F64(3.0)");
    }

    #[test]
    fn parse_div_f64() {
        let tokens = vec![
            test_token!(TokenType::Number("5.0".to_string())),
            test_token!(TokenType::Operator("/".to_string())),
            test_token!(TokenType::Number("3.0".to_string())),
            test_token!(TokenType::Separator(';'))
        ];
        let mut pos = 0;
        let expr = super::MathExpr::parse(&tokens, &mut pos);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(expr.to_string(), "DIVIDE F64(5.0), F64(3.0)");
    }
}
