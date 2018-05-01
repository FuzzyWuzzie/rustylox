use ::opcodes::*;
use ::values::{Value, ValueArray};

pub struct Chunk {
    pub code: Vec<u8>,
    pub count: usize,
    pub capacity: usize,
    pub constants: ValueArray,
    pub lines: Vec<usize>
}

impl Chunk {
    pub fn init() -> Chunk {
        Chunk {
            count: 0,
            capacity: 0,
            code: Vec::new(),
            constants: ValueArray::init(),
            lines: Vec::new()
        }
    }

    pub fn free(&mut self) {
        self.count = 0;
        self.capacity = 0;
        self.code.resize(0, 0);
        self.constants.free();
        self.lines.resize(0, 0);
    }

    fn grow_capacity(capacity:usize) -> usize {
        if capacity < 8 {
            8
        }
        else {
            capacity * 2
        }
    }

    pub fn write(&mut self, byte:u8, line:usize) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = Self::grow_capacity(old_capacity);
            self.code.resize(self.capacity, 0);
            self.lines.resize(self.capacity, 0);
        }

        self.code[self.count] = byte;
        self.lines[self.count] = line;
        self.count += 1;
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write(value);
        self.constants.count - 1
    }

    pub fn _disassemble_chunk(&self, name:&str) {
        println!("== {} ==", name);
        
        let mut i:usize = 0;
        while i < self.count {
            i = self.disassemble_instruction(i);
        }
    }

    fn simple_instruction(name:&str, offset:usize) -> usize {
        println!("{: >16}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant: u8 = self.code[offset + 1];
        println!("{: >16} {:04} '{}'", name, constant, self.constants.values[constant as usize]);
        offset + 2
    }

    pub fn disassemble_instruction(&self, offset:usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        }
        else {
            print!("{:04} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction {
            OP_RETURN => Self::simple_instruction("OP_RETURN", offset),
            OP_CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }
}