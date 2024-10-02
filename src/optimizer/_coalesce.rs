//! Coalesces adjacent instructions

use crate::instruction::IntermediateInstruction;

/// Coalesce adjacent instructions
pub fn coalesce(instrs: Vec<IntermediateInstruction>) -> (Vec<IntermediateInstruction>, bool) {
    let mut new_instrs = vec![];
    let mut changed = false;

    for instr in instrs {
        match instr {
            IntermediateInstruction::Loop(sub_instrs) => {
                let (new_sub_instrs, new_changed) = coalesce(sub_instrs);
                if new_changed { changed = true; }
                new_instrs.push(IntermediateInstruction::Loop(new_sub_instrs));
            }
            IntermediateInstruction::Move(new_offset) => {
                if new_instrs.is_empty() {
                    new_instrs.push(instr.clone());
                } else {
                    let last_ind = new_instrs.len() - 1;
                    if let IntermediateInstruction::Move(offset) = new_instrs[last_ind] {
                        new_instrs[last_ind] = IntermediateInstruction::Move(offset + new_offset);
                        changed = true;
                    } else {
                        new_instrs.push(instr.clone());
                    }
                }
            }
            IntermediateInstruction::Add(new_offset) => {
                if new_instrs.is_empty() {
                    new_instrs.push(instr.clone());
                } else {
                    let last_ind = new_instrs.len() - 1;
                    if let IntermediateInstruction::Add(offset) = new_instrs[last_ind] {
                        new_instrs[last_ind] = IntermediateInstruction::Add(offset + new_offset);
                        changed = true;
                    } else {
                        new_instrs.push(instr.clone());
                    }
                }
            }
            _ => new_instrs.push(instr.clone()),
        }
    }

    (new_instrs, changed)
}
