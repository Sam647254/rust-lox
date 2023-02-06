use super::token::{Token, TokenType};

pub struct Scanner<'a> {
   current: usize,
   start: usize,
   line: usize,
   source: &'a str,
}

impl<'a> Scanner<'a> {
   pub fn new(source: &str) -> Scanner {
      Scanner {
         current: 0,
         start: 0,
         line: 1,
         source
      }
   }

   pub fn scan_token(&mut self) -> Token {
      self.start = self.current;
      if self.is_at_end() {
         return self.make_token(TokenType::EOF);
      }

      self.error_token("Unexpected character".to_string())
   }

   fn is_at_end(&self) -> bool {
      self.current == self.source.len()
   }

   fn make_token(&self, token_type: TokenType<'a>) -> Token {
      Token {
         token_type,
         lexeme: &self.source[self.start..self.current+1],
         line: self.line
      }
   }

   fn error_token(&self, message: String) -> Token {
      self.make_token(TokenType::Error(message.to_string()))
   }
}