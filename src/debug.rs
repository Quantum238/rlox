use chunk::*;
use value::*;

pub fn disassembleChunk(chunk: &Chunk, name: &str){
	println!("=={:?}==", name);
	let mut i:usize = 0;
	loop {
		i = disassembleInstruction(chunk, i);
		if i >= chunk.code.len(){
			break;
		}
	}

}

pub fn disassembleInstruction(chunk: &Chunk, offset: usize) -> usize{
	print!("{:?} ", offset);
	if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1]{
		print!("   | ");
	} else {
		print!("{:?} ",chunk.lines[offset]);
	}
	let ref instruction = chunk.code[offset];
	match *instruction {
	    OpCode::OP_RETURN => simpleInstruction("OP_RETURN", offset),
	    OpCode::OP_CONSTANT => constantInstruction("OP_CONSTANT", &chunk, offset),
	    _ => {
	    	println!("Unknown opcode {:?}", instruction);
	    	offset + 1
	    }
	}
}

fn simpleInstruction(name: &str, offset: usize) -> usize{
	print!("{:?}\n", name);
	offset + 1
}

fn constantInstruction(name: &str, chunk: &Chunk, offset: usize) -> usize{
	let ref constant: OpCode = chunk.code[offset + 1];
	if let OpCode::INDEX(idx) = *constant{
		print!("{:?} {:?} '",name, idx);
		printValue(chunk.constants.values[idx]);
		print!("'\n");
		offset + 2
	} else {
	    unimplemented!();
	}
}

