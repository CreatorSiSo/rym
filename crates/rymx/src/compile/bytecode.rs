pub struct ByteCode {
	opcodes: Vec<OpCode>,
}

impl ByteCode {
	pub fn new() -> Self {
		Self { opcodes: vec![] }
	}

	pub fn write(&mut self, instr: OpCode) {
		self.opcodes.push(instr);
	}

	pub fn get(&self, index: usize) -> OpCode {
		self.opcodes[index]
	}

	pub fn instructions(&self) -> &[OpCode] {
		&self.opcodes
	}
}

impl std::fmt::Debug for ByteCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Chunk {\n")?;

		for (offset, instr) in self.opcodes.iter().enumerate() {
			writeln!(f, "  {offset:04x?}: {instr:?}")?;
		}

		f.write_str("}")
	}
}

pub type Register = u8;
pub const REGISTERS_LENGTH: usize = Register::MAX as usize + 1;
pub type LiteralInt = i16;
pub type LiteralId = u16;
pub type JumpOffset = i16;

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
	Return,
	LoadInt {
		dest: Register,
		val: LiteralInt,
	},
	Copy {
		dest: Register,
		src: Register,
	},
	Add {
		dest: Register,
		a: Register,
		b: Register,
	},
	IsEqual {
		dest: Register,
		a: Register,
		b: Register,
	},
	IsLessThan {
		dest: Register,
		a: Register,
		b: Register,
	},
	IsGreaterThan {
		dest: Register,
		a: Register,
		b: Register,
	},
	Increment {
		dest: Register,
		by: LiteralInt,
	},
	Jump {
		offset: JumpOffset,
	},
	JumpIfTrue {
		test: Register,
		offset: JumpOffset,
	},
	JumpIfNotTrue {
		test: Register,
		offset: JumpOffset,
	},
}

#[test]
fn test_inst_size() {
	// An Opcode should be 32 bits,
	// anything bigger and some variant has been mis-defined
	assert_eq!(std::mem::size_of::<OpCode>(), 4);
}

#[test]
fn assemble_chunk() {
	let mut chunk = ByteCode::new();

	chunk.write(OpCode::LoadInt { dest: 0, val: 10 });
	chunk.write(OpCode::LoadInt { dest: 1, val: 0 });

	chunk.write(OpCode::Increment { dest: 0, by: -1 });
	chunk.write(OpCode::IsEqual {
		dest: 2,
		a: 0,
		b: 1,
	});
	chunk.write(OpCode::JumpIfTrue {
		test: 2,
		offset: -1,
	});
	chunk.write(OpCode::Return);

	println!("{chunk:#?}");
}
