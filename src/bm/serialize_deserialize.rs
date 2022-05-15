use super::Word;
use crate::instruction::AssemblerOp;
use crate::{Instruction, BM};
use std::collections::HashMap;
use std::io::BufRead;
use std::io::{Read, Write};

pub type LabelTable = HashMap<String, Word>;
pub struct UnresolvedLabel {
    pub addr: Word,
    pub label: String,
}
pub type UnresolvedTable = Vec<UnresolvedLabel>;

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

    pub fn program_from_asm<R>(&mut self, source: R, lt: &mut LabelTable)
    where
        R: Read,
    {
        let mut ut = UnresolvedTable::new();
        self.program.clear();

        // Parse Program from Assembly
        for line in std::io::BufReader::new(source).lines() {
            if let Ok(line) = line {
                // Ignore lines starting with # as comments
                if !line.trim().starts_with("#") {
                    match Instruction::from_asm(&line, &self, &mut ut).unwrap() {
                        AssemblerOp::Inst(inst) => {
                            self.program.push(inst);
                        }
                        AssemblerOp::Label(label, address) => {
                            lt.insert(label, address);
                        }
                    };
                }
            }
        }
        self.program.push(Instruction::Halt); // Mark End Of Program

        // Resolved UnresolvedLabels in assembly code at Jump and JumpIf instructions.
        for ul in ut {
            match &self.program[ul.addr as usize] {
                Instruction::Jump(None) => {
                    self.program[ul.addr as usize] =
                        Instruction::Jump(Some(lt.get(ul.label.as_str()).unwrap().clone()));
                }
                Instruction::JumpIf(None) => {
                    Instruction::Jump(Some(lt.get(ul.label.as_str()).unwrap().clone()));
                }
                i => panic!("{} should not be marked unresolved", &i),
            };
        }
    }

    pub fn program_to_asm<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        for inst in self.program.iter() {
            w.write_fmt(format_args!("{}\n", inst))?;
        }
        Ok(())
    }
}
