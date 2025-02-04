use crate::compiler::Exe;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

enum MatchType {
    Bool,
    String,
    Number,
    //    None,
}

pub(crate) struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    token_str: String,
    pub has_errors: bool,
    pub errors: String,
    // The test_t_stack stack is to parse nested tests.  It is necessary
    // because the tokenizer should change how is scans the matches based
    // on the test type. Therefore we have to remember that
    // what the current test type was in order to change the scanner
    // and pop it off when done with the test.
    test_t_stack: Vec<MatchType>,
    line: usize,
    keywords: HashMap<String, TokenType>,
    //    match_type:MatchType,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Scanner {
        let keywords: HashMap<String, TokenType> = [
            ("null".to_string(), TokenType::Null),
            ("nil".to_string(), TokenType::Nil),
            ("true".to_string(), TokenType::True),
            ("false".to_string(), TokenType::False),
            ("var".to_string(), TokenType::Var),
            ("const".to_string(), TokenType::Const),
            ("-interface-".to_string(), TokenType::InterfaceBlock),
            ("-machine-".to_string(), TokenType::MachineBlock),
            ("-actions-".to_string(), TokenType::ActionsBlock),
            ("-domain-".to_string(), TokenType::DomainBlock),
        ]
        .iter()
        .cloned()
        .collect();

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            token_str: String::new(),
            has_errors: false,
            errors: String::new(),
            test_t_stack: Vec::new(),
            line: 1,
            keywords,
            //     match_type:MatchType::None,
        }
    }

    // NOTE! The self param is NOT &self. That is how
    // the member variable token can move ownership to the
    // caller.
    pub fn scan_tokens(mut self) -> (bool, String, Vec<Token>) {
        // Scan header
        while self.is_whitespace() {
            self.advance();
        }
        if self.peek() == '`' {
            self.sync_start();
            if !self.match_first_header_token() {
                return (self.has_errors, self.errors.clone(), self.tokens);
            }
            self.sync_start();
            while !self.is_at_end() {
                if self.peek() == '`' {
                    self.add_string_token_literal(TokenType::SuperString, TokenLiteral::None);
                    self.sync_start();
                    if self.match_last_header_token() {
                        break;
                    }
                }
                self.advance();
            }
        }

        while !self.is_at_end() {
            self.sync_start();
            self.scan_token();
        }

        // todo: the literal needs to be an optional type of generic object
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            TokenLiteral::None,
            self.line,
            self.start,
            len,
        ));
        (self.has_errors, self.errors.clone(), self.tokens)
    }

    fn is_whitespace(&self) -> bool {
        if self.peek() == ' ' || self.peek() == '\n' || self.peek() == '\r' || self.peek() == '\t' {
            return true;
        }
        false
    }

    fn match_first_header_token(&mut self) -> bool {
        for _i in 0..3 {
            if !self.match_char('`') {
                self.error(self.line, "Malformed header token.");
                return false;
            }
        }
        self.add_string_token_literal(TokenType::ThreeTicks, TokenLiteral::None);

        true
    }

    fn match_last_header_token(&mut self) -> bool {
        for _i in 0..3 {
            if !self.match_char('`') {
                return false;
            }
        }
        self.add_string_token_literal(TokenType::ThreeTicks, TokenLiteral::None);

        true
    }

    fn sync_start(&mut self) {
        self.start = self.current;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LParen),
            ')' => self.add_token(TokenType::RParen),
            '[' => self.add_token(TokenType::LBracket),
            ']' => self.add_token(TokenType::RBracket),
            '|' => {
                if self.match_char('|') {
                    if self.match_char('*') {
                        self.add_token(TokenType::AnyMessage);
                    } else if self.match_char('.') {
                        self.add_token(TokenType::PipePipeDot);
                    } else if self.match_char('[') {
                        self.add_token(TokenType::PipePipeLBracket);
                    } else {
                        self.add_token(TokenType::PipePipe);
                    }
                } else {
                    self.add_token(TokenType::Pipe)
                }
            }
            '*' => self.add_token(TokenType::Star),
            '+' => self.add_token(TokenType::Plus),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '$' => {
                enum StackType {
                    Push,
                    Pop,
                }

                if self.match_char('$') {
                    let st;
                    if self.match_char('[') {
                        if self.match_char('+') {
                            st = StackType::Push;
                        } else if self.match_char('-') {
                            st = StackType::Pop;
                        } else {
                            self.error(self.line, "Unexpected character.");
                            return;
                        }
                        if !self.match_char(']') {
                            self.error(self.line, "Unexpected character.");
                            return;
                        }
                        match st {
                            StackType::Push => {
                                self.add_token(TokenType::StateStackOperationPush);
                                return;
                            }
                            StackType::Pop => {
                                self.add_token(TokenType::StateStackOperationPop);
                                return;
                            }
                        }
                    }
                }

                self.add_token(TokenType::State)
            }
            '^' => self.add_token(TokenType::Caret),
            '>' => {
                if self.match_char('>') {
                    if self.match_char('>') {
                        self.add_token(TokenType::GTx3);
                    } else {
                        self.add_token(TokenType::GTx2);
                    }
                } else if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::GT);
                }
            }
            '<' => {
                if self.match_char('<') {
                    if self.match_char('<') {
                        self.add_token(TokenType::LTx3);
                    } else {
                        self.add_token(TokenType::LTx2);
                    }
                } else if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::LT);
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenType::LogicalAnd)
                } else if self.match_char('|') {
                    self.add_token(TokenType::LogicalXor)
                } else {
                    self.add_token(TokenType::And)
                }
            }
            '?' => {
                if self.match_char('!') {
                    self.add_token(TokenType::BoolTestFalse);
                    // Store the context for the parse
                    self.test_t_stack.push(MatchType::Bool);
                } else if self.match_char('~') {
                    self.add_token(TokenType::StringTest);
                    // Store the context for the parse
                    self.test_t_stack.push(MatchType::String);
                } else if self.match_char('#') {
                    self.add_token(TokenType::NumberTest);
                    // Store the context for the parse
                    self.test_t_stack.push(MatchType::Number);
                } else {
                    self.add_token(TokenType::BoolTestTrue);
                    // Store the context for the parse
                    self.test_t_stack.push(MatchType::Bool);
                }
            }
            '@' => self.add_token(TokenType::At),
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                //    self.line += 1;
            }
            '-' => {
                if !self.block_keyword() {
                    if self.match_char('>') {
                        // -> or ->>
                        if self.match_char('>') {
                            // ->>
                            self.add_token(TokenType::ChangeState);
                        } else {
                            // ->
                            self.add_token(TokenType::Transition);
                        }
                    } else if self.match_char('-') {
                        // --- comment text
                        if self.match_char('-') {
                            self.single_line_comment();
                        } else {
                            self.add_token(TokenType::DashDash);
                        }
                    } else if self.is_digit(self.peek()) {
                        self.number();
                    } else {
                        self.add_token(TokenType::Dash);
                    }
                }
            }
            '{' => {
                if self.match_char('-') {
                    if self.match_char('-') {
                        self.multi_line_comment();
                    } else {
                        panic!("Unexpected character.");
                    }
                } else {
                    self.add_token(TokenType::OpenBrace);
                }
            }
            '}' => {
                self.add_token(TokenType::CloseBrace);
            }
            ':' => {
                if self.match_char(':') {
                    self.add_token(TokenType::TestTerminator);
                    self.test_t_stack.pop();
                } else if self.match_char('>') {
                    self.add_token(TokenType::ElseContinue);
                } else {
                    self.add_token(TokenType::Colon);
                }
            }
            ';' => self.add_token(TokenType::Semicolon),
            '"' => self.string(),
            '`' => self.super_string(),
            '#' => {
                if self.match_char('#') {
                    self.add_token(TokenType::SystemEnd);
                } else if self.match_char('[') {
                    self.add_token(TokenType::OuterAttribute) // #[
                } else if self.match_char('!') {
                    if self.match_char('[') {
                        // #![
                        self.add_token(TokenType::InnerAttribute);
                    } else {
                        self.add_token(TokenType::Error); // #!
                    }
                } else {
                    self.add_token(TokenType::System);
                }
            }
            '=' => {
                if self.match_char('>') {
                    self.add_token(TokenType::Dispatch);
                } else if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equals);
                }
            }
            '/' => {
                if self.match_char('/') {
                    if self.match_char('!') {
                        self.add_token(TokenType::MatchNullString);
                    } else {
                        self.add_token(TokenType::MatchEmptyString);
                    }
                } else {
                    self.add_token_sync_start(TokenType::ForwardSlash);
                    self.scan_match();
                }
            }
            '.' => {
                self.add_token(TokenType::Dot);
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.error(self.line, &format!("Found unexpected character '{}'.", c));
                    self.add_token(TokenType::Error);
                }
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.as_bytes()[self.current] as char;
        if c != expected {
            return false;
        }
        self.current += 1;
        self.token_str = String::from(&self.source[self.start..self.current]);

        true
    }

    // TODO: beware - mixing UTF-8 strings and chars here
    fn advance(&mut self) -> char {
        self.current += 1;
        self.token_str = String::from(&self.source[self.start..self.current]);
        let c: char = self.source.as_bytes()[self.current - 1] as char;
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c: char = self.source.as_bytes()[self.current] as char;
        c
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.as_bytes()[self.current + 1] as char;
    }

    fn is_digit(&self, c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
        }
        while self.is_digit(self.peek()) {
            self.advance();
        }

        let number: f32 = self.source[self.start..self.current].parse().unwrap();
        self.add_token_literal(TokenType::Number, TokenLiteral::Float(number));
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        // See if the identifier is a reserved word.
        let text = &self.source[self.start..self.current].to_owned();

        let kw = &self.keywords.get(text);
        if let Some(keyword) = kw {
            let tok_type = *(*keyword);
            self.add_token(tok_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    // TODO: handle EOF w/ error
    fn single_line_comment(&mut self) {
        if !self.is_at_end() {
            while self.peek() != '\n' {
                self.advance();
            }
            self.add_token(TokenType::SingleLineComment);
        }
    }

    // TODO: handle EOF w/ error
    fn multi_line_comment(&mut self) {
        while !self.is_at_end() {
            while self.peek() != '-' {
                self.advance();
            }
            self.advance();
            if self.peek() != '-' {
                continue;
            }
            self.advance();
            if self.peek() != '}' {
                continue;
            }
            self.advance();

            self.add_token(TokenType::MultiLineComment);
            return;
        }
    }

    fn scan_match(&mut self) {
        match self.test_t_stack.last() {
            Some(MatchType::String) => self.scan_string_match(),
            Some(MatchType::Number) => self.scan_number_match(),
            Some(_) => {}
            None => {}
        }
    }

    // Scan the string looking for the end of the match test ('/')
    // or the end of the current match string ('|').
    // match_string_test -> '/' match_string_pattern ('|' match_string_pattern)* '/'

    fn scan_string_match(&mut self) {
        while self.peek() != '/' {
            if self.peek() == '|' {
                self.add_token_sync_start(TokenType::MatchString);
                self.advance();
                self.add_token_sync_start(TokenType::Pipe);
            }
            self.advance();
        }
        self.add_token_sync_start(TokenType::MatchString);
        self.advance();
        self.add_token_sync_start(TokenType::ForwardSlash);
    }

    // match_number_test -> '/' match_number_pattern ('|' match_number_pattern)* '/'

    fn scan_number_match(&mut self) {
        while self.peek() != '/' {
            if self.peek() == '|' {
                self.number();
                self.advance();
                self.add_token_sync_start(TokenType::Pipe);
            }
            self.advance();
        }
        self.number();

        self.sync_start();
        if !self.match_char('/') {
            // TODO
            panic!("todo");
        }
        self.add_token_sync_start(TokenType::ForwardSlash);
    }

    fn block_keyword(&mut self) -> bool {
        // TODO: handle this:
        // #M1
        //     -in-
        // ##

        let start_pos = self.current;
        // let mut block_name:&str;

        let block_sections = [
            ("interface-", TokenType::InterfaceBlock),
            ("machine-", TokenType::MachineBlock),
            ("actions-", TokenType::ActionsBlock),
            ("domain-", TokenType::DomainBlock),
        ];

        // TODO: this is **horribly** ineffcient.

        for (block_name, token_type) in block_sections.iter() {
            for (i, c) in block_name.chars().enumerate() {
                if !self.match_char(c) {
                    break;
                }
                if i == block_name.len() - 1 {
                    self.add_token(*token_type);
                    return true;
                }
            }

            self.current = start_pos;
        }

        self.current = start_pos;
        false
    }

    fn is_alpha(&self, c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn add_token_sync_start(&mut self, tok_type: TokenType) {
        self.add_token_literal(tok_type, TokenLiteral::None);
        self.sync_start();
    }

    fn add_token(&mut self, tok_type: TokenType) {
        Exe::debug_print(&format!("{:?}", tok_type));
        self.add_token_literal(tok_type, TokenLiteral::None);
    }

    fn add_token_literal(&mut self, tok_type: TokenType, literal: TokenLiteral) {
        let lex = self.source[self.start..self.current].to_owned();
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            tok_type, lex, literal, self.line, self.start, len,
        ));
    }

    fn add_string_token_literal(&mut self, tok_type: TokenType, literal: TokenLiteral) {
        let lex = self.source[self.start + 1..self.current - 1].to_owned();
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            tok_type, lex, literal, self.line, self.start, len,
        ));
    }

    fn error(&mut self, line: usize, error_msg: &str) {
        let error = &format!("Line {} : Error: {}\n", line, error_msg);
        self.has_errors = true;
        self.errors.push_str(error);
    }

    fn string(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();
            if c == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }
            } else if c == '\n' {
                // self.line += 1;
            } else if c == '"' {
                break;
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
        }

        self.advance();
        self.add_string_token_literal(TokenType::String, TokenLiteral::None);
    }

    fn super_string(&mut self) {
        let start_line = self.line;
        while !self.is_at_end() {
            let c = self.peek();
            if c == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }
            } else if c == '\n' {
                // self.line += 1;
            } else if c == '`' {
                break;
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            self.error(start_line, "Unterminated super string.");
            return;
        }

        self.advance();
        self.add_string_token_literal(TokenType::SuperString, TokenLiteral::None);
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum TokenType {
    Eof,
    Identifier,
    State,
    GT,             // >
    GTx2,           // >>
    GTx3,           // >>
    Plus,           // +
    Dash,           // -
    DashDash,       // --
    Star,           // *
    EqualEqual,     // ==
    Bang,           // !
    BangEqual,      // !=
    GreaterEqual,   // >=
    LessEqual,      // <=
    LT,             // <
    LTx2,           // <<
    LTx3,           // <<<
    And,            // &
    Pipe,           // |
    Caret,          // ^
    LogicalAnd,     // &&
    LogicalXor,     // &|
    System,         // #
    SystemEnd,      // ##
    OuterAttribute, // #[
    InnerAttribute, // #![
    InterfaceBlock, // -interface-
    MachineBlock,   // -machine-
    ActionsBlock,   // -actions-
    DomainBlock,    // -domain-
    LParen,
    RParen,
    LBracket,
    RBracket,
    Transition,
    ChangeState,
    String,
    ThreeTicks,              // ```
    SuperString,             // `stuff + "stuff"`
    Number,                  // 1, 1.01
    Var,                     // let
    Const,                   // const
    SingleLineComment,       // --- comment
    MultiLineComment,        // {-- comments --}
    OpenBrace,               // {
    CloseBrace,              // }
    True,                    // true
    False,                   // false
    Null,                    // null
    Nil,                     // nil
    Colon,                   // :
    Semicolon,               // ;
    Dispatch,                // =>
    Equals,                  // =
    BoolTestTrue,            // ?
    BoolTestFalse,           // ?!
    StringTest,              // ?~
    NumberTest,              // ?#
    ElseContinue,            // :>
    TestTerminator,          // ::
    ForwardSlash,            // /
    MatchString,             // /<string>/ - contains <string>
    MatchNullString,         // //!
    MatchEmptyString,        // //
    StateStackOperationPush, // $$[+]
    StateStackOperationPop,  // $$[-]
    Dot,                     // .
    At,                      // @
    PipePipe,                // ||
    PipePipeDot,             // ||.
    PipePipeLBracket,        // ||[
    AnyMessage,              // ||*
    Error,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {
    //Integer(i32),
    Float(f32),
    // Double(f64),
    None,
}

impl Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    literal: TokenLiteral,
    pub line: usize,
    pub start: usize,
    pub length: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: TokenLiteral,
        line: usize,
        start: usize,
        length: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            start,
            length,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
