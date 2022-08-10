use std::fmt::Debug;
use colored::*;

pub enum ErrorKind {
    LexerError,
    ParserError,
    CompilerError
}

pub trait ErrorComponent {
    fn to_err_string(&self) -> String;
}

pub struct Error {
    kind: ErrorKind,
    message: String,
    components: Vec<Box<dyn ErrorComponent>>
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_short_string())
    }
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        Error {
            kind,
            message,
            components: vec![]
        }
    }

    pub fn with_component(&mut self, component: Box<dyn ErrorComponent>) -> &mut Self {
        self.components.push(component);
        return self;
    }

    pub fn print_err(&self) {
        let mut error_message: String = match self.kind {
            ErrorKind::LexerError => format!("{}: {}", "Lexer error".red().bold(), self.message),
            ErrorKind::ParserError => format!("{}: {}", "Parser error".red().bold(), self.message),
            ErrorKind::CompilerError => format!("{}: {}", "Compiler error".red().bold(), self.message)
        };
        error_message.push('\n');

        for component in self.components.iter() {
            error_message.push_str(component.to_err_string().as_str());
            error_message.push('\n');
        }

        println!("{}", error_message);
        error_message.push('\n');
    }

    pub fn to_short_string(&self) -> String {
        format!("{}", self.message.red())
    }

    pub fn panic(&self) {
        self.print_err();
        panic!("{}", "Compilation failed due to previous error".red().bold());
    }

    pub fn panic_val(&self) -> &Self {
        self.panic();
        return self;
    }

}

#[macro_export()]
macro_rules! error {
    ($kind: expr, $message: literal, $( $component: expr ) *) => {
        {
            let mut err = Error::new($kind, $message.to_string());
            $(err.with_component(Box::new($component));)*
            err
        }
    };
    ($kind: expr, $message: literal) => {
        {
            Error::new($kind, $message.to_string())
        }
    };
}
