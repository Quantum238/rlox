use value::*;


#[derive(Debug, Clone, Copy)]
pub enum OpCode {
	OP_CONSTANT,
	OP_RETURN,


	INDEX(usize),
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: ValueArray,
    pub lines: Vec<i64>,
    // I'm not sure we'll need these fields?  vec keeps track of growing on its own
    // count: i64,
    // capacity: i64,
}

impl Chunk {
	pub fn new() -> Chunk{
		Chunk {
			// count: 0,
			// capacity: 0,
			code: Vec::new(),
			constants: ValueArray::new(),
			lines: Vec::new(),
		}
	}

	pub fn writeChunk(&mut self, byte: OpCode, line: i64){
			self.code.push(byte);
			self.lines.push(line);
			// I think I can ignore the count and capacity bookkeeping
	}

	pub fn freeChunk(&mut self) {
		self.code.clear();
		self.constants.freeValueArray();
		self.lines.clear();
	}

	pub fn addConstant(&mut self, value: Value) -> usize{
		self.constants.writeValueArray(value);
		self.constants.len() - 1
	}
}
