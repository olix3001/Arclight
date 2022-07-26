macro_rules! append_buffer {
    ($buffer:ident, $tokens:ident, $line_no:ident, $column_no:ident) => {
        {
            // Append the buffer to the tokens if it is not empty
            if $buffer.len() > 0 {
                $tokens.push(Token {
                    token_type: TokenType::Identifier($buffer.clone()),
                    line: $line_no,
                    column: $column_no - $buffer.len(),
                });
                $buffer.clear();
            }
        }
    };
}

macro_rules! append_token {
    ($tokens:ident, $token_type:expr, $line_no:ident, $column_no:ident) => {
        {
            // Append the token to the tokens
            $tokens.push(Token {
                token_type: $token_type,
                line: $line_no,
                column: $column_no,
            });
        }
    };
}

pub mod lexer {
    use std::{io::{ BufReader, BufRead }, fs::File};
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TokenType {
        // Single-character tokens.
        Paren(char),
        Brace(char),
        Operator(char),
        Separator(char),
        Quote(char),
        // Identifiers
        Identifier(String),
        // End of file.
        EOF,
    }    
    
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub struct Token {
        pub token_type: TokenType,
        pub line: usize,
        pub column: usize,
    }

    pub fn tokenize(input: &mut BufReader<File>) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut line_no = 1;
        let mut column_no = 1;

        for line in input.lines() {
            let line = line.unwrap();
            let mut buffer = String::new();

            for c in line.chars() {
                match c {
                    // Single-character tokens.
                    '(' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Paren('('), line_no, column_no);
                    }
                    ')' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Paren(')'), line_no, column_no);
                    }
                    '{' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Brace('{'), line_no, column_no);
                    }
                    '}' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Brace('}'), line_no, column_no);
                    }
                    '+' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('+'), line_no, column_no);
                    }
                    '-' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('-'), line_no, column_no);
                    }
                    '*' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('*'), line_no, column_no);
                    }
                    '/' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('/'), line_no, column_no);
                    }
                    '%' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('%'), line_no, column_no);
                    }
                    '!' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('!'), line_no, column_no);
                    }
                    '=' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('='), line_no, column_no);
                    },
                    '<' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('<'), line_no, column_no);
                    }
                    '>' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Operator('>'), line_no, column_no);
                    }
                    ',' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Separator(','), line_no, column_no);
                    }
                    '.' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Separator('.'), line_no, column_no);
                    }
                    ';' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Separator(';'), line_no, column_no);
                    }
                    ':' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Separator(':'), line_no, column_no);
                    }
                    '\'' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Quote('\''), line_no, column_no);
                    }
                    '"' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        append_token!(tokens, TokenType::Quote('"'), line_no, column_no);
                    }

                    ' ' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                    }
                    '\t' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                    }
                    '\r' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                    }

                    '\n' => {
                        append_buffer!(buffer, tokens, line_no, column_no);
                        line_no += 1;
                        column_no = 1;
                    }

                    // Multi character identifiers
                    _ => {
                        buffer.push(c);
                    }
                }
                column_no += 1;
            }

            line_no += 1;
            column_no = 1;
        }

        // Append EOF token
        append_token!(tokens, TokenType::EOF, line_no, column_no);

        return tokens;
    }
}