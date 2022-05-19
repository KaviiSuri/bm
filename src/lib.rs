pub mod instruction;
pub mod interpreter;
pub mod serialize_deserialize;
pub use instruction::Instruction;

use std::io::Write;

/// Represents the maximum capacity of the evaluation stack.
pub const BM_STACK_CAPACITY: usize = 1024;
/// Represents the maximum capacity of the instruction list.
pub const BM_PROGRAM_CAPACITY: usize = 1024;

/// A word in the virtual machine. Each element of the evaluation stack as well as the instruction pointer needs to be a Word.
pub type Word = i64;

/// BM represents an instance of the virtual machine with all it's state.
#[derive(Debug)]
pub struct BM {
    /// This is the evaluation stack of the virtual machine
    stack: Vec<Word>,
    /// Tracks if the program halted; as of now, only true if Instruction::Halt was interpreted
    halt: bool,
    /// This is the list of instructions for the virtual machine.
    program: Vec<Instruction>,
    /// IP is the instruction pointer for the virtual machine and represents the instruction that is to be executed next.
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
    /// Checks if the virtual machine is halted
    pub fn is_halted(&self) -> bool {
        self.halt
    }

    // Pushes a single Instruction into the virtual machine program
    pub fn push_inst(&mut self, inst: Instruction) {
        assert!(self.program.len() <= BM_PROGRAM_CAPACITY);
        self.program.push(inst);
    }

    /// Copies the program from a Instruction slice to the virtual machine instruction list.
    pub fn load_program_from_memory(&mut self, program: &[Instruction]) {
        assert!(program.len() <= BM_PROGRAM_CAPACITY);
        self.program.extend_from_slice(program);
    }

    /// Dumps the current state of the stack into a Writer.
    /// ```
    /// use bm::{BM, Instruction};
    /// let mut bm: BM = Default::default();
    /// bm.push_inst(bm::Instruction::Push(1));
    /// bm.push_inst(bm::Instruction::Push(2));
    /// bm.program_to_asm(&mut std::io::stdout()).unwrap();
    /// bm.dump_stack(&mut std::io::stdout());
    /// ```
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
