use crate::token::{self};
use crate::error::error;

pub struct Scanner {
    start: i32,
    current: i32,
    line: u32,
    source: String,
    tokens: Vec<token::Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        return Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        };
    }

    pub fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as i32;
    }

    pub fn scan_tokens(&mut self) -> &Vec<token::Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        return &self.tokens;
    }

    pub fn scan_token(&mut self) -> () {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(token::token_type::LEFT_PAREN, None),
            ')' => self.add_token(token::token_type::RIGHT_PAREN, None),
            '{' => self.add_token(token::token_type::LEFT_BRACE, None),
            '}' => self.add_token(token::token_type::RIGHT_BRACE, None),
            ',' => self.add_token(token::token_type::COMMA, None),
            '.' => self.add_token(token::token_type::DOT, None),
            ';' => self.add_token(token::token_type::SEMICOLON, None),
            '-' => self.add_token(token::token_type::MINUS, None),
            '+' => self.add_token(token::token_type::PLUS, None),
            '*' => self.add_token(token::token_type::STAR, None),
            '!' => {
                if self.match_token('=') {
                    self.add_token(token::token_type::BANG_EQUAL, None);
                } else {
                    self.add_token(token::token_type::BANG, None);
                }
            }
            '>' => {
                if self.match_token('=') {
                    self.add_token(token::token_type::GREATER_EQUAL, None);
                } else {
                    self.add_token(token::token_type::GREATER, None);
                }
            }
            '<' => {
                if self.match_token('=') {
                    self.add_token(token::token_type::LESS_EQUAL, None);
                } else {
                    self.add_token(token::token_type::LESS, None);
                }
            }
            '=' => {
                if self.match_token('=') {
                    self.add_token(token::token_type::EQUAL_EQUAL, None);
                } else {
                    self.add_token(token::token_type::EQUAL, None);
                }
            }
            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(token::token_type::SLASH, None);
                }
            }
            '\n' => self.line += 1,
            '\r' => {}
            ' ' => {}
            '\t' => {}
            '"' => self.string(),

            _ => {
                if c.is_numeric() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    error(self.line as u32, "Unexpected error .");
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphabetic() || self.peek().is_numeric() {
            self.advance();
        }
        let substr = &self.source[self.start as usize..self.current as usize];
        match self.match_string_token(substr) {
            Some(n) => {
                self.add_token(n, None);
            }
            None => self.add_token(token::token_type::IDENTIFIER, None),
        }
    }

    fn match_token(&mut self, c: char) -> bool {
        if self.peek() != c || self.is_at_end() {
            return false;
        }
        self.current += 1;
        return true;
    }

    // @TODO use a hashmap instead
    fn match_string_token(&self, str: &str) -> Option<token::token_type> {
        return match str {
            "and" => Some(token::token_type::AND),
            "or" => Some(token::token_type::OR),
            "fun" => Some(token::token_type::FUN),
            "class" => Some(token::token_type::CLASS),
            "if" => Some(token::token_type::IF),
            "else" => Some(token::token_type::ELSE),
            "true" => Some(token::token_type::TRUE),
            "false" => Some(token::token_type::FALSE),
            "super" => Some(token::token_type::SUPER),
            "return" => Some(token::token_type::RETURN),
            "this" => Some(token::token_type::THIS),
            "var" => Some(token::token_type::VAR),
            "while" => Some(token::token_type::WHILE),
            "for" => Some(token::token_type::FOR),
            "print" => Some(token::token_type::PRINT),
            _ => None,
        };
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current as usize)
            .expect("Something went wrong");
        self.current = self.current + 1;
        return c;
    }

    fn add_token(&mut self, token_type: token::token_type, literal: Option<String>) {
        let substr: &str = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(token::Token::new(
            literal.unwrap_or(String::new()),
            self.line,
            token_type,
            substr.to_string(),
        ))
    }

    fn peek(&self) -> char {
        return self
            .source
            .chars()
            .nth((self.current) as usize)
            .unwrap_or('\0');
    }

    fn peek_next(&self) -> char {
        return self
            .source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap_or('\0');
    }

    fn string(&mut self) -> () {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // handle error
            println!("Unterminated string.")
        }

        self.advance();

        let substring = &self.source[(self.start + 1) as usize..(self.current - 1) as usize];
        self.add_token(token::token_type::STRING, Some(substring.to_string()));
    }

    fn number(&mut self) -> () {
        while self.peek().is_numeric(){
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let substring = &self.source[(self.start) as usize..(self.current) as usize];
        self.add_token(token::token_type::NUMBER, Some(substring.to_string()));
    }
}
