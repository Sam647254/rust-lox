use std::fmt::{self, Display};

pub struct Token<'a> {
   pub token_type: TokenType<'a>,
   pub lexeme: &'a str,
   pub line: usize,
}

impl Token<'_> {
   pub fn is_eof(&self) -> bool {
      self.token_type == TokenType::EOF
   }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType<'a> {
   LeftParen,
   RightParen,
   LeftBrace,
   RightBrace,
   Comma,
   Dot,
   Minus,
   Plus,
   Semicolon,
   Slash,
   Star,
   Bang,
   BangEqual,
   Equal,
   DoubleEqual,
   Greater,
   GreaterOrEqual,
   Less,
   LessOrEqual,
   Identifier(&'a str),
   String(&'a str),
   Number(f64),
   And,
   Class,
   Else,
   False,
   For,
   Fun,
   If,
   Nil,
   Or,
   Print,
   Return,
   Super,
   This,
   True,
   Var,
   While,
   EOF,
   Error(String),
}

impl Display for TokenType<'_> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:?}", self)
   }
}
