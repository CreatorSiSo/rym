use crate::compile::{
	bytecode::{ByteCode, JumpOffset, OpCode, REGISTERS_LENGTH},
	Value,
};

struct VirtualMachine {
	pointer: usize,
	chunk: ByteCode,
	registers: [Value; REGISTERS_LENGTH],
}

impl VirtualMachine {
	pub fn new(chunk: ByteCode) -> Self {
		Self {
			pointer: 0,
			chunk,
			registers: [Value::Unit; 256],
		}
	}

	pub fn interpret(&mut self) {
		self.pointer = 0;
		while self.pointer < self.chunk.instructions().len() {
			self.advance();
		}
	}

	fn advance(&mut self) {
		match self.chunk.instructions()[self.pointer] {
			OpCode::Return => {
				// noop for now
			}
			OpCode::LoadInt { dest, val } => {
				self.registers[dest as usize] = Value::Int(val as i64);
			}
			OpCode::Copy { dest, src } => {
				self.registers[dest as usize] = self.registers[src as usize];
			}
			OpCode::Add { dest, a, b } => {
				self.registers[dest as usize] =
					match (self.registers[a as usize], self.registers[b as usize]) {
						(Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs + rhs),
						(Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs + rhs),
						_ => todo!("Will be caught by compiler"),
					}
			}
			OpCode::Increment { dest, by } => match &mut self.registers[dest as usize] {
				Value::Int(inner) => *inner += by as i64,
				_ => todo!(),
			},
			OpCode::IsEqual { dest, a, b } => {
				self.registers[dest as usize] =
					Value::Int((self.registers[a as usize] == self.registers[b as usize]) as i64)
			}
			OpCode::IsLessThan { dest, a, b } => {
				let result = match (self.registers[a as usize], self.registers[b as usize]) {
					(Value::Int(lhs), Value::Int(rhs)) => lhs < rhs,
					(Value::Float(lhs), Value::Float(rhs)) => lhs < rhs,
					_ => todo!(),
				};
				self.registers[dest as usize] = Value::Int(result as i64)
			}
			OpCode::IsGreaterThan { dest, a, b } => {
				let result = match (self.registers[a as usize], self.registers[b as usize]) {
					(Value::Int(lhs), Value::Int(rhs)) => lhs > rhs,
					(Value::Float(lhs), Value::Float(rhs)) => lhs > rhs,
					_ => todo!(),
				};
				self.registers[dest as usize] = Value::Int(result as i64)
			}
			OpCode::Jump { offset } => {
				self.jump(offset);
				return;
			}
			OpCode::JumpIfTrue { test, offset } => match self.registers[test as usize] {
				Value::Int(0) => { /*  noop, test is false */ }
				Value::Int(1) => {
					self.jump(offset);
					return;
				}
				_ => todo!(),
			},
			OpCode::JumpIfNotTrue { test, offset } => match self.registers[test as usize] {
				Value::Int(0) => {
					self.jump(offset);
					return;
				}
				Value::Int(1) => { /*  noop, test is false */ }
				_ => todo!(),
			},
		}
		self.pointer += 1;
	}

	fn jump(&mut self, offset: JumpOffset) {
		if offset.is_negative() {
			self.pointer -= offset.abs() as usize;
		} else {
			self.pointer += offset.abs() as usize;
		}
	}
}

#[test]
fn simple_loop() {
	let mut chunk = ByteCode::new();

	chunk.write(OpCode::LoadInt { dest: 0, val: 10 });
	chunk.write(OpCode::LoadInt { dest: 1, val: 0 });

	chunk.write(OpCode::Increment { dest: 0, by: -1 });
	chunk.write(OpCode::IsGreaterThan {
		dest: 2,
		a: 0,
		b: 1,
	});
	chunk.write(OpCode::JumpIfTrue {
		test: 2,
		offset: -2,
	});

	let mut vm = VirtualMachine::new(chunk);
	vm.interpret();

	assert_eq!(vm.pointer, 5);
	assert_eq!(vm.registers[0], Value::Int(0));
	assert_eq!(vm.registers[1], Value::Int(0));
	assert_eq!(vm.registers[2], Value::Int(0));
}
