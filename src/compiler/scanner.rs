use std::{str::Chars, iter::Peekable};

use super::token::{Token, TokenType};

pub struct Scanner<'a> {
   pub current: usize,
   pub start: usize,
   pub line: usize,
   source: &'a str,
   chars: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
   pub fn new(source: &str) -> Scanner {
      Scanner {
         current: 0,
         start: 0,
         line: 1,
         source,
         chars: source.chars().peekable()
      }
   }

   pub fn scan_token(&mut self) -> Token {
      self.skip_whitespace();
      self.start = self.current;

      match self.advance() {
         Some(c) => match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
               if self.expect('=') {
                  self.make_token(TokenType::BangEqual)
               } else {
                  self.make_token(TokenType::Bang)
               }
            },
            '=' => {
               if self.expect('=') {
                  self.make_token(TokenType::DoubleEqual)
               } else {
                  self.make_token(TokenType::Equal)
               }
            },
            '<' => {
               if self.expect('=') {
                  self.make_token(TokenType::LessOrEqual)
               } else {
                  self.make_token(TokenType::Less)
               }
            },
            '>' => {
               if self.expect('=') {
                  self.make_token(TokenType::GreaterOrEqual)
               } else {
                  self.make_token(TokenType::Greater)
               }
            },
            '"' => self.string(),
            c if c.is_digit(10) => self.number(),
            _ => self.error_token("Unexpected character".to_string())
         }
         None => self.make_token(TokenType::EOF)
      }
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

   fn advance(&mut self) -> Option<char> {
      self.current += 1;
      self.chars.next()
   }

   fn expect(&mut self, expected: char) -> bool {
      if self.chars.next_if_eq(&expected).is_some() {
         self.current += 1;
         true
      } else {
         false
      }
   }

   fn skip_whitespace(&mut self) {
      while let Some(&c) = self.chars.peek() {
         match c {
            ' ' | '\r' | '\t' => self.advance().unwrap(),
            '\n' => {
               self.line += 1;
               self.advance().unwrap()
            },
            '/' if self.chars.clone().nth(1) == Some('c') => {
               self.current += 2;
               self.chars.nth(2).unwrap()
            },
            _ => break,
         };
      }
   }

   fn string(&mut self) -> Token {
      loop {
         let next = self.chars.peek();
         match next {
            Some(&c) => {
               if c == '\n' { return self.error_token("Unterminated string".to_string()); }
               if c == '"' {
                  self.advance().unwrap();
                  return self.make_token(TokenType::String(&self.source[self.start..self.current]));
               }
            },
            None => return self.error_token("Unterminated string".to_string())
         }
      }
   }

   fn number(&mut self) -> Token {
      while let Some(&c) = self.chars.peek() {
         if c.is_digit(10) {
            self.advance();
         }
      }

      match self.chars.peek() {
         Some('.') => {
            if self.chars.clone().nth(1).map(|c| c.is_digit(10)).unwrap_or(false) {
               self.advance();
            }
         },
         _ => (),
      };

      self.make_token(TokenType::Number(self.source[self.start..self.current].parse().unwrap()))
   }
}