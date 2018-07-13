use chunk::*;
use value::*;
use debug::disassembleInstruction;



#[derive(Debug)]
pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
}

pub enum InterpretResult{
	INTERPRET_OK,
	INTERPRET_COMPILE_ERROR,
	INTERPRET_RUNTIME_ERROR,
}

impl VM{
	
	pub fn read_byte(&mut self) -> OpCode{
		let maybe_instruction = self.chunk.code.get(self.ip);
		if let Some(instruction) = maybe_instruction{
			self.ip += 1;
			*instruction
		} else {
		    unimplemented!();
		}


	}
	pub fn read_constant(&mut self) -> Value{
		let maybe_instruction = self.chunk.code.get(self.ip);
		if let Some(&OpCode::INDEX(idx)) = maybe_instruction {
			self.ip += 1;
			self.chunk.constants.values[idx]
		} else {
		    unimplemented!();
		}
	}

	pub fn run(&mut self) -> Option<InterpretResult>{
		loop {
			if cfg!(feature = "qq"){
				disassembleInstruction(&self.chunk, self.ip);
			}
			let instruction = self.read_byte();
			match instruction {
				OpCode::OP_RETURN => {
					return Some(InterpretResult::INTERPRET_OK)
				},
				OpCode::OP_CONSTANT => {
					let constant = self.read_constant();
					printValue(constant);
					print!("\n");
					return None
					// break;
				},
				_ => return Some(InterpretResult::INTERPRET_COMPILE_ERROR),

			};
		};
	}


	pub fn interpret(chunk: Chunk) -> Option<InterpretResult>{
		let mut vm = VM{
			chunk: chunk,
			ip:0
		};
		// self.chunk = chunk;
		// self.ip = 0;
		vm.run()
	}
}
