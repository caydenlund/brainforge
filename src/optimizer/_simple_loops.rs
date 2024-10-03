//! Simple loops optimization: identifies and simplifies simple loops

use crate::instruction::IntermediateInstruction;
use std::collections::HashMap;

/// Convert a loop into a simple loop, if possible
fn make_simple_loop(instr: &IntermediateInstruction) -> Option<IntermediateInstruction> {
    if let IntermediateInstruction::Loop(instrs) = instr {
        let mut current_position: i32 = 0;
        let mut pairs: HashMap<i32, i32> = HashMap::new();
        for instr in instrs {
            match instr {
                IntermediateInstruction::SimpleLoop(sub_pairs) => {
                    for (pair_move, pair_add) in sub_pairs {
                        pairs.insert(
                            current_position + pair_move,
                            pairs.get(&(current_position + pair_move)).unwrap_or(&0) + pair_add,
                        );
                    }
                }
                IntermediateInstruction::Move(offset) => current_position += offset,
                IntermediateInstruction::Add(offset) => {
                    pairs.insert(
                        current_position,
                        pairs.get(&current_position).unwrap_or(&0) + offset,
                    );
                }
                _ => return None,
            }
        }
        if current_position == 0 {
            if let Some(total_add) = pairs.get(&0) {
                if *total_add == 1 {
                    return Some(IntermediateInstruction::SimpleLoop(
                        pairs
                            .iter()
                            .map(|item| (*item.0, -*item.1))
                            .collect::<Vec<(i32, i32)>>(),
                    ));
                } else if *total_add == -1 {
                    return Some(IntermediateInstruction::SimpleLoop(
                        pairs
                            .iter()
                            .map(|item| (*item.0, *item.1))
                            .collect::<Vec<(i32, i32)>>(),
                    ));
                }
            }
        }
        None
    } else {
        None
    }
}

/// Apply simple loops
pub fn make_simple_loops(
    instrs: Vec<IntermediateInstruction>,
) -> (Vec<IntermediateInstruction>, bool) {
    let mut new_instrs = vec![];
    let mut changed = false;

    for instr in instrs {
        match instr {
            IntermediateInstruction::Loop(_) => {
                if let Some(new_instr) = make_simple_loop(&instr) {
                    new_instrs.push(new_instr);
                    changed = true;
                } else {
                    new_instrs.push(instr);
                }
            }
            _ => new_instrs.push(instr),
        }
    }

    (new_instrs, changed)
}
