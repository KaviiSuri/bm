use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::bm::Word;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Push(Word),
    Dup(Word),
    Plus,
    Minus,
    Div,
    Mult,
    Jump(Word),
    JumpIf(Word),
    Eq,
    Halt,
    PrintDebug,
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Nop
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => write!(f, "Instruction::Nop"),
            Instruction::Push(op) => write!(f, "Instruction::Push({})", op),
            Instruction::Plus => write!(f, "Instruction::Plus"),
            Instruction::Minus => write!(f, "Instruction::Minus"),
            Instruction::Div => write!(f, "Instruction::Div"),
            Instruction::Mult => write!(f, "Instruction::Mult"),
            Instruction::Jump(addr) => write!(f, "Instruction::Jump({})", addr),
            Instruction::JumpIf(addr) => write!(f, "Instruction::JumpIf({})", addr),
            Instruction::Eq => write!(f, "Instruction::Eq"),
            Instruction::Halt => write!(f, "Instruction::Halt"),
            Instruction::PrintDebug => write!(f, "Instruction::PrintDebug"),
            Instruction::Dup(addr) => write!(f, "Instruction::Dup({})", addr),
        }
    }
}

#[derive(Debug)]
pub enum InstructionParseErr {
    EmptyLine,
    OperandNotFound(String),
    InvalidOperand(String),
    InvalidInstruction(String),
}

impl Display for InstructionParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionParseErr::EmptyLine => {
                write!(f, "Could not translate empty line to instruction")
            }
            InstructionParseErr::OperandNotFound(l) => {
                write!(f, "Operand required but not found: {}", &l)
            }
            InstructionParseErr::InvalidOperand(l) => {
                write!(f, "Invalid operand (should be a Word): {}", &l)
            }
            InstructionParseErr::InvalidInstruction(l) => {
                write!(f, "Invalid instruction: {}", &l)
            }
        }
    }
}

impl Instruction {
    pub fn from_asm(line: &str) -> Result<Self, InstructionParseErr> {
        let line = line.trim_start();
        if line.len() == 0 {
            return Err(InstructionParseErr::EmptyLine);
        }
        let mut line_iter = line.split_whitespace();
        let name = line_iter.next().unwrap();

        // ==== Instruction::Push ===============
        match name {
            "push" => match line_iter.next() {
                Some(op) => match op.parse::<Word>() {
                    Ok(op) => Ok(Self::Push(op)),
                    Err(_) => Err(InstructionParseErr::InvalidOperand(line.to_string())),
                },
                None => Err(InstructionParseErr::OperandNotFound(line.to_string())),
            },
            "dup" => match line_iter.next() {
                Some(op) => match op.parse::<Word>() {
                    Ok(op) => Ok(Self::Dup(op)),
                    Err(_) => Err(InstructionParseErr::InvalidOperand(line.to_string())),
                },
                None => Err(InstructionParseErr::OperandNotFound(line.to_string())),
            },
            "plus" => Ok(Self::Plus),
            "jmp" => match line_iter.next() {
                Some(op) => match op.parse::<Word>() {
                    Ok(op) => Ok(Self::Jump(op)),
                    Err(_) => Err(InstructionParseErr::InvalidOperand(line.to_string())),
                },
                None => Err(InstructionParseErr::OperandNotFound(line.to_string())),
            },
            _ => Err(InstructionParseErr::InvalidInstruction(line.to_string())),
        }
    }
}
