use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{bm::Word, BM};

use super::serialize_deserialize::BasmCtx;

type Address = Option<Word>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Push(Word),
    Dup(Word),
    Plus,
    Minus,
    Div,
    Mult,
    Jump(Address),
    JumpIf(Address),
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
            Instruction::Nop => write!(f, "nop"),
            Instruction::Push(op) => write!(f, "push {}", op),
            Instruction::Plus => write!(f, "plus"),
            Instruction::Minus => write!(f, "minus"),
            Instruction::Div => write!(f, "div"),
            Instruction::Mult => write!(f, "mult"),
            Instruction::Jump(addr) => write!(f, "jmp {}", addr.unwrap()),
            Instruction::JumpIf(addr) => write!(f, "jmp_if({})", addr.unwrap()),
            Instruction::Eq => write!(f, "eq"),
            Instruction::Halt => write!(f, "halt"),
            Instruction::PrintDebug => write!(f, "print_debug"),
            Instruction::Dup(addr) => write!(f, "dup {}", addr),
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
    pub fn from_asm(
        line: &str,
        bm: &BM,
        ctx: &mut BasmCtx,
    ) -> Result<Instruction, InstructionParseErr> {
        let line = line.trim_start();
        if line.len() == 0 {
            return Err(InstructionParseErr::EmptyLine);
        }
        let (comment_removed_line, _) = line.split_once("#").unwrap_or((line, ""));
        let mut line_iter = comment_removed_line.split_whitespace();
        let mut name = line_iter.next().unwrap();

        if name.len() > 0 && name.ends_with(":") {
            ctx.insert_label(
                name[0..name.len() - 1].to_string(),
                bm.program.len() as Word,
            );
            name = line_iter.next().unwrap();
        }

        match name {
            "nop" => Ok(Self::Nop),
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
            "minus" => Ok(Self::Minus),
            "div" => Ok(Self::Div),
            "mult" => Ok(Self::Mult),
            "jmp" => match line_iter.next() {
                Some(op) => match op.parse::<Word>() {
                    Ok(op) => Ok(Self::Jump(Some(op))),
                    Err(_) => {
                        ctx.add_deffered_opperand(bm.program.len() as Word, op.to_string());
                        Ok(Self::Jump(None))
                    }
                },
                None => Err(InstructionParseErr::OperandNotFound(line.to_string())),
            },
            "jmpif" => match line_iter.next() {
                Some(op) => match op.parse::<Word>() {
                    Ok(op) => Ok(Self::JumpIf(Some(op))),
                    Err(_) => {
                        ctx.add_deffered_opperand(bm.program.len() as Word, op.to_string());
                        Ok(Self::JumpIf(None))
                    }
                },
                None => Err(InstructionParseErr::OperandNotFound(line.to_string())),
            },
            "eq" => Ok(Self::Eq),
            "halt" => Ok(Self::Halt),
            _ => Err(InstructionParseErr::InvalidInstruction(line.to_string())),
        }
    }
}
