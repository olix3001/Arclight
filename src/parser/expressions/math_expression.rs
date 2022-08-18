
use inkwell::values::{AnyValueEnum, IntValue};

use crate::{error, utils::error::{Error, ErrorKind}, lexer::lexer::TokenType};

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
            panic!("LHS ERROR (TODO: implement this better)")
        }
        let rhs = self.rhs.generate(context, module, builder, scope_manager);
        if rhs.is_none() {
            panic!("RHS ERROR (TODO: implement this better)")
        }

        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();
        if lhs.is_int_value() {
            return Some(AnyValueEnum::IntValue(match self.operation {
                MathOperation::ADD => builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "iaddtmp"),
                MathOperation::SUBTRACT => builder.build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "isubtmp"),
                MathOperation::MULTIPLY => builder.build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "imultmp"),
                MathOperation::DIVIDE => builder.build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "isdivtmp")
            }))
        } else if lhs.is_float_value() {
            todo!("Implement float math")
        }

        todo!("Implement some error")
    }

    fn to_string(&self) -> String {
        format!("{:?} {}, {}", self.operation, self.lhs.to_string(), self.rhs.to_string())
    }
}
