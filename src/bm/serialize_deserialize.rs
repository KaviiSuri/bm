use std::io::BufRead;
use std::io::{Read, Write};

use crate::{Instruction, BM};

impl BM {
    pub fn serialize_program_into<W>(&self, w: W)
    where
        W: Write,
    {
        bincode::serialize_into(w, &self.program).expect("could not serialzie");
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
                if !line.starts_with("#") {
                    v.push(Instruction::from_asm(&line).unwrap());
                }
            }
        }
        v.push(Instruction::Halt);
        v
    }

    pub fn program_to_asm<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        for &inst in &self.program {
            w.write_fmt(format_args!("{}\n", inst))?;
        }
        Ok(())
    }
}
