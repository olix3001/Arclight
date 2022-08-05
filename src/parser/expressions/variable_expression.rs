

// use inkwell::values::AnyValueEnum;

// use crate::lexer::lexer::TokenType;

// use super::{Parseable, ASTExpr, data_types::ToAny, Scope};

// pub struct VariableCallExpr {
//     name: String,
// }

// impl Parseable for VariableCallExpr {
//     fn parse(tokens: &Vec<crate::lexer::lexer::Token>, pos: &mut usize) -> Result<Box<dyn super::ASTExpr>, String> {
//         let name = &tokens[*pos].token_type;
//         match name {
//             TokenType::Identifier(ref s) => {
//                 *pos += 1;
//                 return Ok(Box::new(VariableCallExpr {
//                     name: s.clone(),
//                 }));
//             },
//             _ => return Err(format!("Expected identifier, found {:?}", tokens[*pos]))
//         }
//     }
// }

// impl ASTExpr for VariableCallExpr {
//     fn generate<'a>(&self, context: &'a inkwell::context::Context, module: &inkwell::module::Module<'a>, builder: &inkwell::builder::Builder<'a>, scope: Option<&Scope>) -> Option<inkwell::values::AnyValueEnum<'a>> {
//         let var = scope.unwrap().get_variable(&self.name);
//         if var.is_none() {
//             return None;
//         }
//         let var = var.unwrap();
//         let var = *var;
//         let load = builder.build_load(var, &self.name);
//         let load = load.to_any();
//         Some(load)
//     }
 
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }
