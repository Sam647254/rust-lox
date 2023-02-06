use crate::compiler::token::TokenType;

use self::scanner::Scanner;

mod scanner;
mod token;

pub fn compile(source: &str) {
   let mut scanner = Scanner::new(source);
   let mut line = 0;
   loop {
      let token = scanner.scan_token();
      if token.line != line {
         print!("{: >4} ", token.line);
         line = token.line;
      } else {
         print!("   | ");
      }
      println!("{: >10} '{}'", token.token_type, token.lexeme);

      if token.token_type == TokenType::EOF {
         break;
      }
   }
   todo!()
}