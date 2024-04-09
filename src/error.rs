use crate::token::{ self, Token};

pub static mut HAD_ERROR: bool = false;

fn report(line: u32, location: &str, message: &str) -> () {
    eprintln!("[ line {} ] Error {}: {}", line, location, message);
    unsafe { HAD_ERROR = true };
}

pub fn error(line: u32, message: &str) {
    report(line, "", &message);
}

pub fn token_error(token:&Token, message:&str) {
if token.token_type == token::token_type::EOF {
      report(token.line," at end", message);
    } else {
      let error_line: String = " at '".to_string() + &token.lexeme + "'";
      report(token.line, &error_line, message);
    }
}

pub enum ErrorTypes {
    UnexpectedTokenError
}
