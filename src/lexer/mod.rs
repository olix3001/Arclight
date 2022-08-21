
pub mod lexer {
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TokenType {
        // Single-character tokens.
        Paren(char),
        Brace(char),
        Operator(String),
        Separator(char),
        // Identifiers
        Identifier(String),
        // Numbers
        Number(String),
        // Strings
        String(String),
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

    #[derive(Debug, Clone)]
    enum LexerState {
        Start,
        Identifier,
        Number,
        String,
        Comment,
        Operator,
        Error,
    }
    
    macro_rules! add_token {
        ($self: expr, $t: expr, $ns: expr) => {{
            $self.tokens.push(Token {
                token_type: $t,
                line: $self.line_no,
                column: $self.column_no,
            });
            $self.state = $ns;
        }};
        ($self: expr, $ns: expr) => {
            $self.state = $ns
        };
        ($self: expr, $ns: expr, $c: expr ;) => {{
            $self.curr_token.push($c);
            $self.state = $ns
        }};
    }
    struct Lexer {
        tokens: Vec<Token>,
        line_no: usize,
        column_no: usize,
        curr_token: String,
        state: LexerState,
    }

    impl Lexer{

        pub fn new(state: LexerState) -> Self {
            Self {
                tokens: vec![],
                line_no: 0,
                column_no: 0,
                curr_token: String::new(),
                state
            }
        }

        pub fn feed(&mut self, c: char) {
            // change line and/or column
            if c == '\n' {
                self.line_no += 1;
                self.column_no = 0;
            } else {
                self.column_no += 1;
            }

            // handle state
            match self.state.clone() {
                LexerState::Start => {
                    // Number token
                    if c.is_numeric() {
                        add_token!(self, LexerState::Number, c ;);
                    } else {
                        // Handle single tokens
                        match c {
                            // Single char tokens
                            '(' => add_token!(self, TokenType::Paren('('), LexerState::Start),
                            ')' => add_token!(self, TokenType::Paren(')'), LexerState::Start),
                            '{' => add_token!(self, TokenType::Brace('{'), LexerState::Start),
                            '}' => add_token!(self, TokenType::Brace('}'), LexerState::Start),
                            ':' => add_token!(self, TokenType::Separator(':'), LexerState::Start),
                            ',' => add_token!(self, TokenType::Separator(','), LexerState::Start),
                            '.' => add_token!(self, TokenType::Separator('.'), LexerState::Start),
                            ';' => add_token!(self, TokenType::Separator(';'), LexerState::Start),
                            '=' => add_token!(self, LexerState::Operator, c ;),
                            '+' => add_token!(self, LexerState::Operator, c ;),
                            '-' => add_token!(self, LexerState::Number, c ;),
                            '*' => add_token!(self, LexerState::Operator, c ;),
                            '%' => add_token!(self, LexerState::Operator, c ;),
                            '!' => add_token!(self, LexerState::Operator, c ;),
                            '<' => add_token!(self, LexerState::Operator, c ;),
                            '>' => add_token!(self, LexerState::Operator, c ;),
                            '&' => add_token!(self, LexerState::Operator, c ;),
                            '|' => add_token!(self, LexerState::Operator, c ;),

                            // More complex tokens
                            '/' => add_token!(self, TokenType::Operator("/".to_string()), LexerState::Start),
                            '#' => add_token!(self, LexerState::Comment),

                            '"' => add_token!(self, LexerState::String, c ;),

                            ' ' | '\t' | '\n' | '\r' => (),
                            _ => add_token!(self, LexerState::Identifier, c ;),
                        }
                    }
                }
                LexerState::Identifier => {
                    // Identifier
                    match c {
                        '_' | 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => {
                            self.curr_token.push(c);
                        }
                        _ => {
                            add_token!(self, TokenType::Identifier(self.curr_token.clone()), LexerState::Start);
                            self.feed(c);
                            self.curr_token.clear();
                        }
                    }
                }
                LexerState::Number => {
                    // Number
                    match c {
                        '0' ..= '9' | '.' | 'f' | 'u' | 'i' => {
                            self.curr_token.push(c);
                        }
                        _ => {
                            if self.curr_token.len() == 1 && self.curr_token.chars().next().unwrap() == '-' {
                                add_token!(self, TokenType::Operator(self.curr_token.clone()), LexerState::Start);
                                self.curr_token.clear();
                                self.feed(c);
                                return;
                            }
                            add_token!(self, TokenType::Number(self.curr_token.clone()), LexerState::Start);
                            self.curr_token.clear();
                            self.feed(c);
                        }
                    }
                }
                LexerState::String => {
                    // String
                    match c {
                        '"' => {
                            self.curr_token.push(c);
                            add_token!(self, TokenType::String(self.curr_token.clone()), LexerState::Start); 
                        },
                        _ => {
                            self.curr_token.push(c);
                        }
                    }
                }
                LexerState::Comment => {
                    // Ingore this, It's just a comment
                    match c {
                        '\n' => self.state = LexerState::Start,
                        _ => {}
                    }
                }
                LexerState::Operator => {
                    // Operator
                    match c {
                        '=' => { self.curr_token.push(c); }
                        '+' => { self.curr_token.push(c); }
                        '-' => { self.curr_token.push(c); }
                        '*' => { self.curr_token.push(c); }
                        '%' => { self.curr_token.push(c); }
                        '!' => { self.curr_token.push(c); }
                        '<' => { self.curr_token.push(c); }
                        '>' => { self.curr_token.push(c); }
                        '&' => { self.curr_token.push(c); }
                        '|' => { self.curr_token.push(c); }
                        _ => {
                            add_token!(self, TokenType::Operator(self.curr_token.clone()), LexerState::Start);
                            self.feed(c);
                            self.curr_token.clear();
                        }
                    }
                }
                LexerState::Error => {
                    
                }
            }
        }

        pub fn feed_str(&mut self, string: &str) {
            for c in string.chars() {
                self.feed(c);
            }
        }

    }

    
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(LexerState::Start);
        lexer.feed_str(input);
        lexer.tokens.push(
            Token {
                token_type: TokenType::EOF,
                line: lexer.line_no,
                column: lexer.column_no,
            }
        );
        lexer.tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::{TokenType, tokenize};

    #[test]
    fn lex_single_ident() {
        let input = "abc;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Identifier("abc".to_string()));
    }

    #[test]
    fn lex_single_number() {
        let input = "123;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number("123".to_string()));
    }

    #[test]
    fn lex_single_string() {
        let input = "\"abc\";";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::String("\"abc\"".to_string()));
    }

    #[test]
    fn lex_single_comment() {
        let input = "#abc";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn lex_single_operator() {
        let input = "=;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Operator("=".to_string()));
    }

    #[test]
    fn lex_double_operator() {
        let input = "==;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Operator("==".to_string()));
    }

    #[test]
    fn lex_import_statement() {
        let input = "import hello::world { print, println };";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 12);

        assert_eq!(tokens[0].token_type, TokenType::Identifier("import".to_string()), "Expected 'import' keyword");
        assert_eq!(tokens[1].token_type, TokenType::Identifier("hello".to_string()), "Expected 'hello' identifier");
        assert_eq!(tokens[2].token_type, TokenType::Separator(':'), "Expected ':' separator");
        assert_eq!(tokens[3].token_type, TokenType::Separator(':'), "Expected second ':' separator");
        assert_eq!(tokens[4].token_type, TokenType::Identifier("world".to_string()), "Expected 'world' identifier");
        assert_eq!(tokens[5].token_type, TokenType::Brace('{'), "Expected '{{' brace");
        assert_eq!(tokens[6].token_type, TokenType::Identifier("print".to_string()), "Expected 'print' identifier");
        assert_eq!(tokens[7].token_type, TokenType::Separator(','), "Expected ',' separator");
        assert_eq!(tokens[8].token_type, TokenType::Identifier("println".to_string()), "Expected 'println' identifier");
        assert_eq!(tokens[9].token_type, TokenType::Brace('}'), "Expected '}}' brace");
        assert_eq!(tokens[10].token_type, TokenType::Separator(';'), "Expected ';'");
    }
}
