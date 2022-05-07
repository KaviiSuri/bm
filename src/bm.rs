use std::io::Write;

use crate::Instruction;

pub const BM_STACK_CAPACITY: usize = 1024;
pub const BM_PROGRAM_CAPACITY: usize = 1024;

pub type Word = i64;

pub mod interpreter;
pub mod serialize_deserialize;

#[derive(Debug)]
pub struct BM {
    stack: Vec<Word>,
    halt: bool,
    program: Vec<Instruction>,
    ip: Word,
}

impl Default for BM {
    fn default() -> Self {
        let mut stack = Vec::new();
        stack.reserve(BM_STACK_CAPACITY);

        let mut program = Vec::new();
        program.reserve(BM_PROGRAM_CAPACITY);

        Self {
            stack,
            halt: Default::default(),
            program,
            ip: Default::default(),
        }
    }
}

impl BM {
    pub fn is_halted(&self) -> bool {
        self.halt
    }
    pub fn push_inst(&mut self, inst: Instruction) {
        assert!(self.program.len() <= BM_PROGRAM_CAPACITY);
        self.program.push(inst);
    }

    pub fn load_program_from_memory(&mut self, program: &[Instruction]) {
        assert!(program.len() <= BM_PROGRAM_CAPACITY);
        self.program.extend_from_slice(program);
    }

    pub fn dump_stack<W>(&self, f: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        writeln!(f, "Stack: ")?;
        if self.stack.len() == 0 {
            writeln!(f, "   [empty]")?;
            return Ok(());
        }
        for i in 0..self.stack.len() {
            writeln!(f, "   {}", self.stack[i])?;
        }
        Ok(())
    }
}
