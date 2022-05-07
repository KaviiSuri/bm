use std::io::BufRead;
use std::io::{Read, Write};

use crate::{Instruction, BM};

impl BM {
    pub fn serialize_program_into<W>(w: W, program: &[Instruction])
    where
        W: Write,
    {
        bincode::serialize_into(w, program).expect("could not serialzie");
    }

    pub fn deserialize_program_from<R>(r: R) -> Vec<Instruction>
    where
        R: Read,
    {
        bincode::deserialize_from::<R, Vec<Instruction>>(r).expect("could not deserialize program")
    }

    pub fn from_asm<R>(source: R) -> Vec<Instruction>
    where
        R: Read,
    {
        let mut v = vec![];
        for line in std::io::BufReader::new(source).lines() {
            if let Ok(line) = line {
                v.push(Instruction::from_asm(&line).unwrap());
            }
        }
        v.push(Instruction::Halt);
        v
    }
}