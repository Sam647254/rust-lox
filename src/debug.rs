use crate::{chunk::OpCode, Chunk};

impl Chunk {
   pub fn disassemble(&self, name: &str) {
      println!("== {} ==", name);

      let mut offset = 0;
      while offset < self.code_size() {
         offset = self.disassemble_instruction(offset);
      }
   }

   pub fn disassemble_instruction(&self, offset: usize) -> usize {
      print!("{:0>4} ", offset);
      if offset > 0 && self.line(offset) == self.line(offset - 1) {
         print!("   | ")
      } else {
         print!("{:0>4} ", self.line(offset));
      }
      let instruction: OpCode = self[offset].into();
      match instruction {
         OpCode::Return => simple_instruction("Return", offset),
         OpCode::Constant => self.constant_instruction("Constant", offset),
         OpCode::Negate => simple_instruction("Negate", offset),
         OpCode::Add => simple_instruction("Add", offset),
         OpCode::Subtract => simple_instruction("Subtract", offset),
         OpCode::Multiply => simple_instruction("Multiply", offset),
         OpCode::Divide => simple_instruction("Divide", offset),
         OpCode::Unknown(opcode) => simple_instruction(&format!("Unknown ({})", opcode), offset),
      }
   }

   fn constant_instruction(&self, name: &str, offset: usize) -> usize {
      let constant = self[offset + 1];
      println!(
         "{:<16} {:>4} '{}'",
         name,
         constant,
         self.constant(constant as usize)
      );
      offset + 2
   }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
   println!("{}", name);
   offset + 1
}
