
pub mod lexer {
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TokenType {
        // Single-character tokens.
        Paren(char),
        Brace(char),
        Operator(char),
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

    #[derive(Clone)]
    enum TransitionAction {
        None,
        Push,
        Flush(fn(&str) -> TokenType),
        Feedback,
        Error(String),
    }

    #[derive(Clone)]
    struct Transition {
        pub actions: Vec<TransitionAction>,
        should_transition_for: fn(char) -> bool,
        pub next_state: Box<State>,
    }

    impl Transition {
        pub fn new(should_transition_for: fn(char) -> bool, next_state: Box<State>, actions: Vec<TransitionAction>) -> Transition {
            Transition {
                actions,
                should_transition_for,
                next_state,
            }
        }

        pub fn should_transition_for(&self, c: char) -> bool {
            (self.should_transition_for)(c)
        }
    }

    macro_rules! create_transition {
        ($should_transition: expr, $next_state: expr, $($actions: expr), +) => {
            Box::new(Transition::new($should_transition, $next_state, vec![$($actions), +]))
        }
    }

    #[derive(Clone)]
    struct State {
        pub transitions: Vec<Box<Transition>>,
    }
    impl State {
        pub fn new(transitions: Vec<Box<Transition>>) -> State {
            State {
                transitions,
            }
        }
        pub fn set_transition(&mut self, transitions: Vec<Box<Transition>>) {
            self.transitions = transitions;
        }
    }

    struct Lexer {
        tokens: Vec<Token>,
        line_no: usize,
        column_no: usize,
        curr_token: String,
        state: Box<State>,
    }

    impl Lexer {

        pub fn new(state: Box<State>) -> Self {
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

            for transition in self.state.transitions.clone().iter() {
                if transition.should_transition_for(c) {
                    self.state = transition.next_state.clone();
                    for action in transition.actions.iter() {
                        match action {
                            TransitionAction::None => {},
                            TransitionAction::Push => {
                                self.curr_token.push(c);
                            },
                            TransitionAction::Flush(tok) => {
                                self.tokens.push(Token {
                                    token_type: tok(&self.curr_token),
                                    line: self.line_no,
                                    column: self.column_no,
                                });
                                self.curr_token.clear();
                            },
                            TransitionAction::Feedback => {
                                self.feed(c)
                            },
                            TransitionAction::Error(msg) => {
                                panic!("{} at line {}, column {}", msg, self.line_no, self.column_no);
                            }
                        }
                    }
                    break;
                }
            }
        }

        pub fn feed_str(&mut self, string: &str) {
            for c in string.chars() {
                self.feed(c);
            }
        }
    }

    macro_rules! flush {
        ($token_type: ident) => {
            TransitionAction::Flush(|s: &str| TokenType::$token_type(s.to_string()))
        };
        ($token_type: ident !) => {
            TransitionAction::Flush(|s: &str| TokenType::$token_type(s.chars().next().unwrap()))
        };
    }

    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut start = State::new(vec![]);
        
        let mut string_state = State::new(vec![]);
        let mut ident_state = State::new(vec![]);
        let mut number_state = State::new(vec![]);
        
        start.set_transition(vec![
            create_transition!(|c: char| c == ' ' || c == '\t' || c == '\n' || c == '\r', Box::new(start), TransitionAction::None),
            // TODO: Add comment transition
            // Basic transitions
            create_transition!(|c: char| c == ',', Box::new(start), TransitionAction::Push, flush!(Separator !)),
            create_transition!(|c: char| c == '[', Box::new(start), TransitionAction::Push, flush!(Brace !)),
            create_transition!(|c: char| c == ']', Box::new(start), TransitionAction::Push, flush!(Brace !)),
            create_transition!(|c: char| c == '{', Box::new(start), TransitionAction::Push, flush!(Brace !)),
            create_transition!(|c: char| c == '}', Box::new(start), TransitionAction::Push, flush!(Brace !)),
            create_transition!(|c: char| c == ':', Box::new(start), TransitionAction::Push, flush!(Separator !)),
            create_transition!(|c: char| c == ';', Box::new(start), TransitionAction::Push, flush!(Separator !)),
            create_transition!(|c: char| c == '(', Box::new(start), TransitionAction::Push, flush!(Paren !)),
            create_transition!(|c: char| c == ')', Box::new(start), TransitionAction::Push, flush!(Paren !)),
            create_transition!(|c: char| c == '+', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '-', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '*', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '/', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '%', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '=', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '!', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '<', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '>', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '&', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            create_transition!(|c: char| c == '|', Box::new(start), TransitionAction::Push, flush!(Operator !)),
            
            // Number
            create_transition!(|c: char| c.is_numeric(), Box::new(number_state), TransitionAction::None),
            
            // String
            create_transition!(|c: char| c == '"', Box::new(string_state), TransitionAction::None),
            
            // Char
            // TODO: Add char state
            
            // Ident
            create_transition!(|_c: char| true, Box::new(ident_state), TransitionAction::None),
        ]);
        // Number
        number_state.set_transition(vec![
            create_transition!(|c: char| c.is_numeric(), Box::new(number_state), TransitionAction::Push),
            create_transition!(|c: char| !c.is_numeric(), Box::new(start), flush!(Number)),
        ]);

            // String
        string_state.set_transition(vec![
            create_transition!(|c: char| c != '"', Box::new(string_state), TransitionAction::Push),
            create_transition!(|c: char| c == '"', Box::new(start), flush!(String)),
        ]);

        // Ident
        ident_state.set_transition(vec![
            create_transition!(|c: char| c.is_alphanumeric() || c == '_', Box::new(ident_state), TransitionAction::Push),
            create_transition!(|c: char| !c.is_alphanumeric() && c != '_', Box::new(start), flush!(Identifier)),
        ]);
                
        let mut lexer = Lexer::new(Box::new(start));
        lexer.feed_str(input);
                
        // Add EOF at the end
        lexer.tokens.push(Token {
            token_type: TokenType::EOF,
            line: lexer.line_no,
            column: lexer.column_no,
        });
                
        lexer.tokens
    }
}