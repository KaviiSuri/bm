use std::fmt::Display;

use crate::{Instruction, BM};

use super::{Word, BM_STACK_CAPACITY};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpreterErr {
    StackOverflow,
    StackUnderflow,
    DivideByZero,
    IllegalInstructionAccess(Word),
    IllegalOperand,
}

impl Display for InterpreterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StackOverflow => write!(f, "Err::StackOverflow"),
            Self::StackUnderflow => write!(f, "Err::StackUnderflow"),
            Self::DivideByZero => write!(f, "Err::DivideByZero"),
            Self::IllegalInstructionAccess(ip) => {
                write!(f, "Err::IllegalInstructionAccess({})", ip)
            }
            Self::IllegalOperand => write!(f, "Err::IllegalOperand"),
        }
    }
}

impl BM {
    pub fn execute_program(&mut self, limit: Option<usize>) -> Result<(), InterpreterErr> {
        let mut i = 1;
        while !self.is_halted() {
            match limit {
                Some(l) if l <= i => break,
                _ => {}
            }
            self.execute_instruction()?;
            i += 1;
        }
        Ok(())
    }
    pub fn execute_instruction(&mut self) -> Result<(), InterpreterErr> {
        if self.ip < 0 || self.program.len() as Word <= self.ip {
            return Err(InterpreterErr::IllegalInstructionAccess(self.ip));
        }
        match self.program[self.ip as usize] {
            Instruction::Nop => {
                self.ip += 1;
            }
            Instruction::Push(op) => {
                if self.stack.len() >= BM_STACK_CAPACITY {
                    return Err(InterpreterErr::StackOverflow);
                }
                self.stack.push(op);
                self.ip += 1;
            }
            Instruction::Dup(op) => {
                if self.stack.len() >= BM_STACK_CAPACITY {
                    return Err(InterpreterErr::StackOverflow);
                }
                if self.stack.len() as Word - op <= 0 {
                    return Err(InterpreterErr::StackUnderflow);
                }

                if op < 0 {
                    return Err(InterpreterErr::IllegalOperand);
                }
                self.stack
                    .push(self.stack[self.stack.len() - 1 - op as usize]);
                self.ip += 1
            }
            Instruction::Plus => {
                if self.stack.len() < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                let stack_size = self.stack.len();
                self.stack[stack_size - 2] += self.stack[self.stack.len() - 1];
                self.stack.pop();
                self.ip += 1;
            }
            Instruction::Minus => {
                if self.stack.len() < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                let stack_size = self.stack.len();
                self.stack[stack_size - 2] -= self.stack[self.stack.len() - 1];
                self.stack.pop();
                self.ip += 1;
            }
            Instruction::Mult => {
                if self.stack.len() < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                let stack_size = self.stack.len();
                self.stack[stack_size - 2] *= self.stack[self.stack.len() - 1];
                self.stack.pop();
                self.ip += 1;
            }
            Instruction::Div => {
                if self.stack.len() < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                if self.stack[self.stack.len() - 1] == 0 {
                    return Err(InterpreterErr::DivideByZero);
                }
                let stack_size = self.stack.len();
                self.stack[stack_size - 2] /= self.stack[self.stack.len() - 1];
                self.stack.pop();
                self.ip += 1;
            }
            Instruction::Jump(addr) => {
                self.ip = addr.expect("Address should be a number in interpretter");
            }
            Instruction::JumpIf(addr) => {
                if self.stack.len() < 1 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                if self.stack[self.stack.len() - 1] == 1 {
                    self.stack.pop();
                    self.ip = addr.expect("Address should be a number in interpretter");
                } else {
                    self.ip += 1;
                }
            }
            Instruction::Eq => {
                if self.stack.len() < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                let stack_size = self.stack.len();
                self.stack[stack_size - 2] =
                    (self.stack[stack_size - 1] == self.stack[stack_size - 2]) as Word;
                self.stack.pop();
                self.ip += 1;
            }
            Instruction::Halt => {
                self.halt = true;
            }
            Instruction::PrintDebug => {
                if self.stack.len() < 1 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                println!("{}", self.stack[self.stack.len() - 1]);
                self.stack.pop();
                self.ip += 1;
            }
        };
        Ok(())
    }
}
