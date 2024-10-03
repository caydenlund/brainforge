//! Simple loops optimization: identifies and simplifies simple loops

use crate::instruction::IntermediateInstruction;
use std::collections::HashSet;

fn make_simple_loop(instrs: &Vec<IntermediateInstruction>) -> Option<IntermediateInstruction> {
    let mut current_delta = 0;
    let mut current_offset = 0;

    let mut instructions = vec![];

    let mut zeroes = HashSet::new();

    println!("Input:");
    for instr in instrs {
        println!("    - {:?}", instr);
        match instr {
            IntermediateInstruction::Zero => {
                if current_offset == 0 {
                    return None;
                }
                zeroes.insert(current_offset);
                instructions.extend(vec![
                    IntermediateInstruction::Move(current_offset),
                    IntermediateInstruction::Zero,
                    IntermediateInstruction::Move(-current_offset),
                ]);
            }
            IntermediateInstruction::Move(stride) => {
                current_offset += stride;
            }
            IntermediateInstruction::Add(delta) => {
                if current_offset == 0 {
                    current_delta += delta;
                } else {
                    if zeroes.contains(&current_offset) {
                        return None;
                    }
                    instructions.push(IntermediateInstruction::AddDynamic(current_offset, *delta));
                }
            }
            IntermediateInstruction::AddDynamic(_target, _delta) => {
                return None;
                // if current_offset == 0 {
                //     return None;
                // }
                // let absolute_target = current_offset + target;
                // if absolute_target == 0 {
                //     return None;
                // }
                // instructions.push(IntermediateInstruction::AddDynamic(absolute_target, *delta));
            }
            _ => return None,
        }
    }
    if current_offset != 0 {
        return None;
    }
    let sign = match current_delta {
        -1 => 1,
        1 => -1,
        _ => return None,
    };
    instructions.push(IntermediateInstruction::Zero);

    println!("Output:");
    Some(IntermediateInstruction::SimpleLoop(
        instructions
            .into_iter()
            .map(|instr| {
                println!("    - {:?}", instr);
                match instr {
                    IntermediateInstruction::AddDynamic(target, multiplier) => {
                        IntermediateInstruction::AddDynamic(target, sign * multiplier)
                    }
                    _ => instr,
                }
            })
            .collect::<Vec<IntermediateInstruction>>(),
    ))
}

/// Apply simple loops
pub fn make_simple_loops(
    instrs: Vec<IntermediateInstruction>,
) -> (Vec<IntermediateInstruction>, bool) {
    let mut new_instrs = vec![];
    let mut changed = false;

    for instr in instrs {
        match instr {
            IntermediateInstruction::Loop(sub_instrs) => {
                if let Some(next_instr) = make_simple_loop(&sub_instrs) {
                    new_instrs.push(next_instr);
                    changed = true;
                } else {
                    let (next_instrs, next_changed) = make_simple_loops(sub_instrs);
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
