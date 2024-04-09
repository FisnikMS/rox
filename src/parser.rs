use crate::{
    error::{self, error}, expr::{Expr, Literal}, token::{self, token_type, Token}
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: i32,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Box<Expr> {
        return self.expression();
    }

    fn is_at_end(&self) -> bool {
        return match self.peek() {
            Some(k) => k.token_type == token::token_type::EOF,
            None => true,
        };
    }
    fn peek(&self) -> Option<&token::Token> {
        return self.tokens.get(self.current as usize);
    }

    fn previous(&self) -> Option<&token::Token> {
        return self.tokens.get(self.current as usize - 1);
    }

    fn advance(&mut self) -> Option<&token::Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn match_token(&mut self, token_types: Vec<token_type>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn match_single_token(&mut self, token_types: token_type) -> bool {
        return self.match_token(vec![token_types]);
    }

    fn check(&mut self, token_type: token_type) -> bool {
        if self.is_at_end() {
            return false;
        }
        return match self.peek() {
            Some(n) => n.token_type == token_type,
            None => false,
        };
    }

    // OPERATIONS

    fn expression(&mut self) -> Box<Expr> {
        return self.equality();
    }

    fn primary(&mut self) -> Result<Box<Expr>, String> {
        if self.match_single_token(token_type::TRUE) {
            return Ok(Box::new(Expr::Literal(Literal::True)));
        };
        if self.match_single_token(token_type::FALSE) {
            return Ok(Box::new(Expr::Literal(Literal::False)));
        };
        if self.match_single_token(token_type::NIL) {
            return Ok(Box::new(Expr::Literal(Literal::Nil)));
        };
        if self.match_single_token(token_type::NUMBER) {
            return Ok(Box::new(Expr::Literal(Literal::Number(
                self.previous()
                    .unwrap()
                    .literal
                    .parse::<f64>()
                    .expect("Expected a floating number"),
            ))));
        };
        if self.match_single_token(token_type::STRING) {
            return Ok(Box::new(Expr::Literal(Literal::String(
                self.previous().unwrap().literal.clone(),
            ))));
        };
        if self.match_single_token(token_type::LEFT_PAREN) {
            let expr: Box<Expr> = self.expression();
            return match self.consume(
                token_type::RIGHT_PAREN,
                "Expect ')' after expression.".to_string(),
            ) {
                Ok(_) => Ok(Box::new(Expr::Grouping(expr))),
                Err(n) => Err(n),
            };
        };
        return Err("Unexpected token".to_string());
    }

    // Option would be a better choice
    fn consume(&mut self, token_type: token_type, error_message: String) -> Result<&Token, String> {
        if self.check(token_type) {
            return match self.advance() {
                Some(n) => Ok(n),
                None => Err(error_message),
            };
        }
        return Err(error_message);
    }

    fn comparison(&mut self) -> Box<Expr> {
        let expr: Box<Expr> = self.term();
        while self.match_token(vec![
            token::token_type::GREATER,
            token::token_type::GREATER_EQUAL,
            token::token_type::LESS,
            token::token_type::LESS_EQUAL,
        ]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term();
            return Box::new(Expr::Binary(expr, operator, right));
        }
        return expr;
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_token(vec![token::token_type::BANG, token::token_type::MINUS]) {
            let operator = self.previous().unwrap().clone();
            let right: Box<Expr> = self.unary();
            return Box::new(Expr::Unary(operator, right));
        }
        return self.primary().expect("TEST");
    }

    fn term(&mut self) -> Box<Expr> {
        let expr: Box<Expr> = self.factor();
        while self.match_token(vec![token::token_type::MINUS, token::token_type::PLUS]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor();
            return Box::new(Expr::Binary(expr, operator, right));
        }
        return expr;
    }

    fn factor(&mut self) -> Box<Expr> {
        let expr: Box<Expr> = self.unary();
        while self.match_token(vec![token::token_type::STAR, token::token_type::SLASH]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor();
            return Box::new(Expr::Binary(expr, operator, right));
        }
        return expr;
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.comparison();

        while self.match_token(vec![
            token::token_type::BANG_EQUAL,
            token::token_type::EQUAL_EQUAL,
        ]) {
            let operator = self.previous().unwrap().clone();
            let right: Box<Expr> = self.comparison();
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        return expr;
    }
}
