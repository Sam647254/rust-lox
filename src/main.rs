use chunk::{Chunk, OpCode};
use vm::VM;

mod chunk;
mod debug;
mod vm;

fn test_chunk() -> Chunk {
   let mut chunk = Chunk::new();

   let c_index = chunk.add_constant(1.2);
   chunk.add_code(OpCode::Constant.into(), 123);
   chunk.add_code(c_index as u8, 123);

   let c_index = chunk.add_constant(3.4);
   chunk.add_code(OpCode::Constant.into(), 123);
   chunk.add_code(c_index as u8, 123);

   chunk.add_code(OpCode::Add.into(), 123);

   let c_index = chunk.add_constant(5.6);
   chunk.add_code(OpCode::Constant.into(), 123);
   chunk.add_code(c_index as u8, 123);

   chunk.add_code(OpCode::Divide.into(), 123);
   chunk.add_code(OpCode::Negate.into(), 123);
   chunk.add_code(OpCode::Return.into(), 123);
   chunk
}

fn main() {
   let chunk = test_chunk();
   let mut vm = VM::new(chunk);
   println!("Execution start");
   let result = vm.run();
   match result {
      vm::InterpreterResult::Ok => println!("Execution finished"),
      vm::InterpreterResult::CompileError => todo!(),
      vm::InterpreterResult::RuntimeError => todo!(),
      vm::InterpreterResult::InternalError(message) => panic!("{}", message),
   }
}
