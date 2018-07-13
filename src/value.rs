
pub type Value = f64;

#[derive(Debug)]
pub struct ValueArray {
	pub values: Vec<Value>,
}


impl ValueArray {
	pub fn new() -> ValueArray{
		ValueArray {
			values: Vec::new()
		}
	}

	pub fn writeValueArray(&mut self, value: Value) -> (){
			self.values.push(value);
	}

	pub fn freeValueArray(&mut self) {
		self.values.clear()
	}

	pub fn len(&self) -> usize{
		self.values.len()
	}
}

pub fn printValue(value: Value){
	print!("{:?}", value);
}
