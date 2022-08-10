use crate::{lexer::lexer::Token, utils::error::ErrorComponent};
use colored::*;

pub struct ErrorTokenComponent {
    token: Option<Token>,
    message: String,
}

impl ErrorComponent for ErrorTokenComponent {
    fn to_err_string(&self) -> String {
        if self.token.is_some() {
            let token_value = match self.token.as_ref().unwrap().token_type {
                crate::lexer::lexer::TokenType::EOF => "EOF".green().to_string(),
                crate::lexer::lexer::TokenType::Paren(ref s) => format!("'{}'", s.to_string().green()),
                crate::lexer::lexer::TokenType::Brace(ref s) => format!("'{}'", s.to_string().green()),
                crate::lexer::lexer::TokenType::Number(ref s) => format!("{}", s.green()),
                crate::lexer::lexer::TokenType::String(ref s) => format!("{}", s.green()),
                crate::lexer::lexer::TokenType::Operator(ref s) => format!("'{}'", s.green()),
                crate::lexer::lexer::TokenType::Separator(ref s) => format!("'{}'", s.to_string().green()),
                crate::lexer::lexer::TokenType::Identifier(ref s) => s.green().to_string()
            };

            let token_str = format!("{} {} {} {} {} {}",
                                    "found".yellow(),
                                    token_value,
                                    "at line".yellow(),
                                    self.token.as_ref().unwrap().line.to_string().green(),
                                    "and column".yellow(),
                                    self.token.as_ref().unwrap().column.to_string().green()
                                    );
            format!("{} - {}: {}", "Token error".red().bold(), self.message.red(), token_str)
        } else {
            format!("{} - {}", "Token error".red().bold(), self.message.red())
        }
    }
}

impl ErrorTokenComponent {
    pub fn new(message: String, token: Option<Token>) -> ErrorTokenComponent {
        ErrorTokenComponent {
            message,
            token
        }
    }
}
