use std::ops::Index;

#[repr(u8)]
pub enum OpCode {
   Return,
   Constant,
   Negate,
   Add,
   Subtract,
   Multiply,
   Divide,
   Unknown(u8),
}

pub struct Chunk {
   code: Vec<u8>,
   constants: Vec<f64>,
   lines: Vec<i32>,
}

impl Chunk {
   pub fn new() -> Chunk {
      Chunk {
         code: Vec::new(),
         constants: Vec::new(),
         lines: Vec::new(),
      }
   }

   pub fn add_code(&mut self, code: u8, line: i32) {
      self.code.push(code);
      self.lines.push(line);
   }

   pub fn add_constant(&mut self, constant: f64) -> usize {
      self.constants.push(constant);
      self.constants.len() - 1
   }

   pub fn code_size(&self) -> usize {
      self.code.len()
   }

   pub fn constant(&self, index: usize) -> &f64 {
      &self.constants[index]
   }

   pub fn line(&self, index: usize) -> &i32 {
      &self.lines[index]
   }
}

impl Index<usize> for Chunk {
   type Output = u8;

   fn index(&self, index: usize) -> &Self::Output {
      &self.code[index]
   }
}

impl Into<OpCode> for u8 {
   fn into(self) -> OpCode {
      match self {
         0 => OpCode::Return,
         1 => OpCode::Constant,
         2 => OpCode::Negate,
         3 => OpCode::Add,
         4 => OpCode::Subtract,
         5 => OpCode::Multiply,
         6 => OpCode::Divide,
         _ => OpCode::Unknown(self),
      }
   }
}

impl Into<u8> for OpCode {
   fn into(self) -> u8 {
      match self {
         OpCode::Return => 0,
         OpCode::Constant => 1,
         OpCode::Negate => 2,
         OpCode::Add => 3,
         OpCode::Subtract => 4,
         OpCode::Multiply => 5,
         OpCode::Divide => 6,
         OpCode::Unknown(opcode) => opcode,
      }
   }
}
