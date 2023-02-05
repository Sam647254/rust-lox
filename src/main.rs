use std::{env::args, process::ExitCode, io::{self, stdin, Read}, fs::File};

use chunk::{Chunk, OpCode};
use vm::VM;

mod chunk;
mod compiler;
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

fn main() -> ExitCode {
   let args: Vec<String> = args().collect();
   let result = match args.len() {
      1 => run_repl(),
      2 => run_file(&args[1]),
      _ => {
         println!("Usage: lox [path]");
         return ExitCode::from(64);
      }
   };
   match result {
      vm::InterpreterResult::Ok => ExitCode::SUCCESS,
      vm::InterpreterResult::CompileError => ExitCode::from(74),
      vm::InterpreterResult::RuntimeError => todo!(),
      vm::InterpreterResult::InternalError(message) => panic!("{}", message),
   }
}

fn run_repl() -> vm::InterpreterResult {
   let mut line = String::new();
   let mut vm = vm::VM::new_empty();
   print!("> ");
   while let Ok(_) = stdin().read_line(&mut line) {
      match vm.interpret(&line) {
        vm::InterpreterResult::Ok => todo!(),
        vm::InterpreterResult::CompileError => todo!(),
        vm::InterpreterResult::RuntimeError => todo!(),
        vm::InterpreterResult::InternalError(_) => todo!(),
    }
   }
   vm::InterpreterResult::Ok
}

fn run_file(filename: &str) -> vm::InterpreterResult {
   match File::open(filename) {
      Ok(mut file) => {
         let mut source = String::new();
         file.read_to_string(&mut source).expect("Could not read file");
         todo!()
      }
      Err(_err) => vm::InterpreterResult::CompileError
   }
}