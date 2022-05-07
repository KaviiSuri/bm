use std::io::Write;

use crate::Instruction;

pub const BM_STACK_CAPACITY: usize = 1024;
pub const BM_PROGRAM_CAPACITY: usize = 1024;

pub type Word = i64;

pub mod interpreter;
pub mod serialize_deserialize;

#[derive(Debug)]
pub struct BM {
    stack: [Word; BM_STACK_CAPACITY],
    stack_size: usize,
    halt: bool,
    program: [Instruction; BM_PROGRAM_CAPACITY],
    program_size: usize,
    ip: Word,
}

impl Default for BM {
    fn default() -> Self {
        Self {
            stack: [0; BM_STACK_CAPACITY],
            stack_size: Default::default(),
            halt: Default::default(),
            program: [Default::default(); BM_PROGRAM_CAPACITY],
            program_size: Default::default(),
            ip: Default::default(),
        }
    }
}

impl BM {
    pub fn is_halted(&self) -> bool {
        self.halt
    }
    pub fn push_inst(&mut self, inst: Instruction) {
        assert!(self.program_size <= BM_PROGRAM_CAPACITY);
        self.program[self.program_size] = inst;
        self.program_size += 1;
    }

    pub fn load_program_from_memory(&mut self, program: &[Instruction]) {
        assert!(program.len() <= BM_PROGRAM_CAPACITY);
        self.program[..program.len()].copy_from_slice(program);
        self.program_size = program.len();
    }

    pub fn dump_stack<W>(&self, f: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        writeln!(f, "Stack: ")?;
        if self.stack_size == 0 {
            writeln!(f, "   [empty]")?;
            return Ok(());
        }
        for i in 0..self.stack_size {
            writeln!(f, "   {}", self.stack[i])?;
        }
        Ok(())
    }
}
