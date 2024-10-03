//! Simple loops optimization: identifies and simplifies simple loops

use crate::instruction::IntermediateInstruction;
use std::collections::HashMap;

/// Convert a loop into a simple loop, if possible
fn make_simple_loop(instr: &IntermediateInstruction) -> Option<IntermediateInstruction> {
    if let IntermediateInstruction::Loop(instrs) = instr {
        let mut total_move: i32 = 0;
        let mut total_add: i32 = 0;
        let mut pairs: HashMap<i32, i32> = HashMap::new();
        for instr in instrs {
            match instr {
                IntermediateInstruction::SimpleLoop(_) => todo!(),
                IntermediateInstruction::Move(offset) => total_move += offset,
                IntermediateInstruction::Add(offset) => {
                    if total_move == 0 {
                        total_add += offset;
                    } else {
                        pairs.insert(total_move,
                                     pairs.get(&total_move).unwrap_or(&0) + offset);
                    }
                }
                _ => return None,
            }
        }
        let instr_to_str =
            |instr: &IntermediateInstruction| {
                match instr {
                    IntermediateInstruction::Move(offset) => {
                        if *offset > 0 {
                            ">".repeat(*offset as usize)
                        } else {
                            "<".repeat(-*offset as usize)
                        }
                    }
                    IntermediateInstruction::Add(offset) => {
                        if *offset > 0 {
                            "+".repeat(*offset as usize)
                        } else {
                            "-".repeat(-*offset as usize)
                        }
                    }
                    _ => "@".into(),
                }
            };
        if total_move == 0 && (total_add == 1 || total_add == -1) {
            println!("Simple loop: [{}]", instrs.iter().map(instr_to_str).collect::<Vec<String>>().join(""));
            for (pair_move, pair_add) in pairs.iter() { println!("    - ({}, {})", pair_move, pair_add); }
            return Some(IntermediateInstruction::SimpleLoop(pairs.iter().map(|item| (*item.0, *item.1)).collect::<Vec<(i32, i32)>>()));
        } else {
            println!("Non-simple loop: {}, {}, [{}]", total_move, total_add, instrs.iter().map(instr_to_str).collect::<Vec<String>>().join(""));
        }
        None
        // todo!()
    } else {
        None
    }
}

/// Apply simple loops
pub fn make_simple_loops(instrs: Vec<IntermediateInstruction>) -> (Vec<IntermediateInstruction>, bool) {
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
