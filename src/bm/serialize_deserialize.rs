use super::Word;
use crate::{Instruction, BM};
use std::collections::HashMap;
use std::io::BufRead;
use std::io::{Read, Write};

pub struct UnresolvedLabel {
    pub addr: Word,
    pub label: String,
}

pub struct BasmCtx {
    label_table: HashMap<String, Word>,
    deferred_operand: Vec<UnresolvedLabel>,
}

impl Default for BasmCtx {
    fn default() -> Self {
        Self {
            label_table: Default::default(),
            deferred_operand: Default::default(),
        }
    }
}

impl BasmCtx {
    pub fn insert_label(&mut self, label: String, addr: Word) {
        self.label_table.insert(label, addr);
    }

    pub fn add_deffered_opperand(&mut self, addr: Word, label: String) {
        self.deferred_operand.push(UnresolvedLabel { addr, label });
    }

    pub fn get_addr_for(&self, label: &str) -> Word {
        self.label_table.get(label).unwrap().clone()
    }
}

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

    pub fn program_from_asm<R>(&mut self, source: R, ctx: &mut BasmCtx)
    where
        R: Read,
    {
        self.program.clear();

        // Parse Program from Assembly
        for line in std::io::BufReader::new(source).lines() {
            if let Ok(line) = line {
                // Ignore lines starting with # as comments
                if !line.trim().starts_with("#") {
                    self.program
                        .push(Instruction::from_asm(&line, &self, ctx).unwrap());
                }
            }
        }
        self.program.push(Instruction::Halt); // Mark End Of Program

        for ul in &ctx.deferred_operand {
            match &self.program[ul.addr as usize] {
                Instruction::Jump(None) => {
                    self.program[ul.addr as usize] =
                        Instruction::Jump(Some(ctx.get_addr_for(ul.label.as_str())));
                }
                Instruction::JumpIf(None) => {
                    self.program[ul.addr as usize] =
                        Instruction::Jump(Some(ctx.get_addr_for(ul.label.as_str())));
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
