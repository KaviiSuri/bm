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
    pub fn execute(&mut self) -> Result<(), InterpreterErr> {
        if self.ip < 0 || self.program_size as Word <= self.ip {
            return Err(InterpreterErr::IllegalInstructionAccess(self.ip));
        }
        match self.program[self.ip as usize] {
            Instruction::Nop => {
                self.ip += 1;
            }
            Instruction::Push(op) => {
                if self.stack_size >= BM_STACK_CAPACITY {
                    return Err(InterpreterErr::StackOverflow);
                }
                self.stack[self.stack_size] = op;
                self.stack_size += 1;
                self.ip += 1;
            }
            Instruction::Dup(op) => {
                if self.stack_size >= BM_STACK_CAPACITY {
                    return Err(InterpreterErr::StackOverflow);
                }
                if self.stack_size as Word - op <= 0 {
                    return Err(InterpreterErr::StackUnderflow);
                }

                if op < 0 {
                    return Err(InterpreterErr::IllegalOperand);
                }
                self.stack[self.stack_size] = self.stack[self.stack_size - 1 - op as usize];
                self.stack_size += 1;
                self.ip += 1
            }
            Instruction::Plus => {
                if self.stack_size < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                self.stack[self.stack_size - 2] += self.stack[self.stack_size - 1];
                self.stack_size -= 1;
                self.ip += 1;
            }
            Instruction::Minus => {
                if self.stack_size < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                self.stack[self.stack_size - 2] -= self.stack[self.stack_size - 1];
                self.stack_size -= 1;
                self.ip += 1;
            }
            Instruction::Mult => {
                if self.stack_size < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                self.stack[self.stack_size - 2] *= self.stack[self.stack_size - 1];
                self.stack_size -= 1;
                self.ip += 1;
            }
            Instruction::Div => {
                if self.stack_size < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                if self.stack[self.stack_size - 1] == 0 {
                    return Err(InterpreterErr::DivideByZero);
                }
                self.stack[self.stack_size - 2] /= self.stack[self.stack_size - 1];
                self.stack_size -= 1;
                self.ip += 1;
            }
            Instruction::Jump(addr) => {
                self.ip = addr;
            }
            Instruction::JumpIf(addr) => {
                if self.stack_size < 1 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                if self.stack[self.stack_size - 1] == 1 {
                    self.stack_size -= 1;
                    self.ip = addr;
                } else {
                    self.ip += 1;
                }
            }
            Instruction::Eq => {
                if self.stack_size < 2 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                self.stack[self.stack_size - 2] =
                    (self.stack[self.stack_size - 1] == self.stack[self.stack_size - 2]) as Word;
                self.stack_size -= 1;
                self.ip += 1;
            }
            Instruction::Halt => {
                self.halt = true;
            }
            Instruction::PrintDebug => {
                if self.stack_size < 1 {
                    return Err(InterpreterErr::StackUnderflow);
                }
                println!("{}", self.stack[self.stack_size - 1]);
                self.stack_size -= 1;
                self.ip += 1;
            }
        };
        Ok(())
    }
}
