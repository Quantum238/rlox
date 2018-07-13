#![allow(non_camel_case_types, non_snake_case)]


mod chunk;
mod debug;
mod value;
mod vm;
use chunk::*;
use debug::*;
use value::*;
use vm::*;


fn main(){
    let mut chunk = Chunk::new();
    // let mut vm = VM::new();

    let constant = chunk.addConstant(1.2);
    chunk.writeChunk(OpCode::OP_CONSTANT, 123);
    chunk.writeChunk(OpCode::INDEX(constant), 123);
    chunk.writeChunk(OpCode::OP_RETURN, 123);

    // disassembleChunk(&chunk, "test chunk");

    VM::interpret(chunk);



    // vm.freeVM();
    // chunk.freeChunk();

}
