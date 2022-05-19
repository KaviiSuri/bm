use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Word, BM};

use super::serialize_deserialize::BasmCtx;

/// An address is the operand of instruction like Jmp and JmpIf.
/// These can either a Word or None
type Address = Option<Word>;

/// Instruction represents a singular operation that the virtual machine executes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// No Operation
    Nop,
    /// Push the operand on stack
    Push(Word),
    /// Duplicate the element which is "operand" places from the top
    Dup(Word),
    /// Add the top 2 elements of the stack
    Plus,
    /// Subtract the top element of the stack from the one after it
    Minus,
    /// Divide the second top element by the top element of the stack.
    Div,
    /// Multiply the top 2 elements of the stack
    Mult,
    /// Jump to an address
    Jump(Address),
    /// Jump to given address if the top of stack isn't zero
    JumpIf(Address),
    /// Check if top 2 elements of the stack are equal
    Eq,
    /// Halt program execution
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

/// Err Generated when parsing instructions
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

/// Parse a singular instruction from assembly text.
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
