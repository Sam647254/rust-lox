use std::{
   collections::{btree_map::Values, LinkedList},
   ops::{Add, Div, Mul, Sub},
};

use crate::chunk::{Chunk, OpCode};

pub struct VM {
   chunk: Chunk,
   ip: usize,
   stack: LinkedList<f64>,
}

pub enum InterpreterResult {
   Ok,
   CompileError,
   RuntimeError,
   InternalError(String),
}

impl VM {
   pub fn new(chunk: Chunk) -> VM {
      VM {
         chunk,
         ip: 0,
         stack: LinkedList::new(),
      }
   }

   pub fn run(&mut self) -> InterpreterResult {
      loop {
         #[cfg(feature = "debug_disassemble")]
         {
            self.print_stack();
            self.chunk.disassemble_instruction(self.ip);
         }
         let instruction: OpCode = self.read_byte().into();
         match instruction {
            OpCode::Return => {
               match self.stack.pop_back() {
                  Some(value) => println!("Result: {}", value),
                  None => (),
               }
               return InterpreterResult::Ok;
            }
            OpCode::Negate => match self.stack.pop_back() {
               Some(value) => self.stack.push_back(-value),
               None => {
                  return InterpreterResult::InternalError(String::from(
                     "Empty stack during negate",
                  ))
               }
            },
            OpCode::Constant => {
               let constant = *self.read_constant();
               self.stack.push_back(constant);
            }
            OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
               let op = match instruction {
                  OpCode::Add => Add::add,
                  OpCode::Subtract => Sub::sub,
                  OpCode::Multiply => Mul::mul,
                  OpCode::Divide => Div::div,
                  _ => unreachable!(),
               };
               match self.binary_op(&op) {
                  Some(_) => (),
                  None => return InterpreterResult::InternalError(String::from("Not enough operands")),
               }
            }
            OpCode::Unknown(opcode) => {
               return InterpreterResult::InternalError(format!("Unknown opcode: {}", opcode))
            }
         }
      }
   }

   fn binary_op(&mut self, op: &dyn Fn(f64, f64) -> f64) -> Option<()> {
      let b = self.stack.pop_back()?;
      let a = self.stack.pop_back()?;
      self.stack.push_back(op(b, a));
      Some(())
   }

   fn read_byte(&mut self) -> u8 {
      let byte = self.chunk[self.ip];
      self.ip += 1;
      byte
   }

   fn read_constant(&mut self) -> &f64 {
      let index = self.read_byte();
      self.chunk.constant(index.into())
   }

   fn print_stack(&mut self) {
      print!("          ");
      for value in &self.stack {
         print!("[ {} ]", value);
      }
      println!();
   }
}
