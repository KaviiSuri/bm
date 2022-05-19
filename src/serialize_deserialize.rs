use super::Word;
use crate::{Instruction, BM};
use std::collections::HashMap;
use std::io::BufRead;
use std::io::{Read, Write};

/// Placeholder for Address/Label type operands
/// It is used to convert all the occurances of labels being used to raw addresses.
pub struct UnresolvedLabel {
    /// Address of the instruction where label is used
    pub addr: Word,
    /// Label
    pub label: String,
}

/// Context for Basm Parser. Contains everything necessary for the parser to do the parsing.
pub struct BasmCtx {
    /// Table of the labels that are created in the code.
    label_table: HashMap<String, Word>,
    /// i.e. all the occurances of a label being used.
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
    /// Insert a new label
    pub fn insert_label(&mut self, label: String, addr: Word) {
        self.label_table.insert(label, addr);
    }

    /// Insert a new occurance of label being used.
    pub fn add_deffered_opperand(&mut self, addr: Word, label: String) {
        self.deferred_operand.push(UnresolvedLabel { addr, label });
    }

    /// Convert a label to it's raw address.
    pub fn get_addr_for(&self, label: &str) -> Word {
        self.label_table.get(label).unwrap().clone()
    }
}

impl BM {
    /// Serialize the program of the virtual machine into a Writer as binary.
    pub fn serialize_program_into<W>(&self, w: W)
    where
        W: Write,
    {
        bincode::serialize_into(w, &self.program).expect("could not serialzie");
    }

    /// Parse binary data from Reader and convert to List of Instructions
    pub fn deserialize_program_from<R>(r: R) -> Vec<Instruction>
    where
        R: Read,
    {
        bincode::deserialize_from::<R, Vec<Instruction>>(r).expect("could not deserialize program")
    }

    /// Parse program from assembly
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
