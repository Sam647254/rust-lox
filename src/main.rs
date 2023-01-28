use chunk::{Chunk, OpCode};

mod chunk;
mod debug;

fn main() {
   let mut chunk = Chunk::new();
   let c_index = chunk.add_constant(1.2);
   chunk.add_code(OpCode::Constant.into(), 123);
   chunk.add_code(c_index as u8, 123);
   chunk.add_code(OpCode::Return.into(), 123);
   chunk.disassemble("Test chunk");
}
