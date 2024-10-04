use crate::instruction::IntermediateInstruction;

/// Attempt to match a scan and apply it
fn make_scan(instrs: &Vec<IntermediateInstruction>) -> Option<IntermediateInstruction> {
    let mut stride = 0;
    for instr in instrs {
        match instr {
            IntermediateInstruction::Move(shift) => stride += shift,
            _ => return None,
        }
    }

    if !vec![-4, -2, -1, 1, 2, 4].contains(&stride) {
        return None;
    }
    Some(IntermediateInstruction::Scan(stride))
}

/// Find scans and apply them
pub fn make_scans(instrs: Vec<IntermediateInstruction>) -> (Vec<IntermediateInstruction>, bool) {
    let mut new_instrs = vec![];
    let mut changed = false;

    for instr in instrs {
        match instr {
            IntermediateInstruction::Loop(sub_instrs) => {
                if let Some(next_instr) = make_scan(&sub_instrs) {
                    new_instrs.push(next_instr);
                    changed = true;
                } else {
                    let (next_instrs, next_changed) = make_scans(sub_instrs);
                    new_instrs.push(IntermediateInstruction::Loop(next_instrs));
                    if next_changed {
                        changed = true;
                    }
                }
            }
            _ => new_instrs.push(instr),
        }
    }

    (new_instrs, changed)
}
