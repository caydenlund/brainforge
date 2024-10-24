//! Defines the procedures to run a BF program
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use super::RuntimeState;
use crate::instruction::{BasicInstruction, BasicInstructionType};
use libc::c_int;
use std::io::Read;

/// Interprets the given BF instructions
pub fn interpret(src: &Vec<BasicInstruction>, mem_size: usize) {
    let mut state = RuntimeState::new(mem_size);

    while state.instr < src.len() {
        match src[state.instr].instr {
            BasicInstructionType::Left => state.ptr -= 1,
            BasicInstructionType::Right => state.ptr += 1,
            BasicInstructionType::Decr => {
                state.memory[state.ptr] = state.memory[state.ptr].wrapping_sub(1)
            }
            BasicInstructionType::Incr => {
                state.memory[state.ptr] = state.memory[state.ptr].wrapping_add(1)
            }
            BasicInstructionType::Read => unsafe {
                state.memory[state.ptr] = libc::getchar() as u8;
            },
            BasicInstructionType::Write => unsafe {
                libc::putchar(state.memory[state.ptr] as c_int);
            },
            // BasicInstructionType::Read => {
            //     if let Some(Ok(ch)) = std::io::stdin().bytes().next() {
            //         state.memory[state.ptr] = ch;
            //     } else {
            //         state.memory[state.ptr] = 0;
            //     };
            // }
            // BasicInstructionType::Write => print!("{}", state.memory[state.ptr] as char),
            BasicInstructionType::LBrace(instr) => {
                if state.memory[state.ptr] == 0 {
                    state.instr = instr
                }
            }
            BasicInstructionType::RBrace(instr) => {
                if state.memory[state.ptr] != 0 {
                    state.instr = instr
                }
            }
        }
        state.instr += 1;
    }
}

/// Interprets the given BF instructions, with added profiling
pub fn interpret_profile(src: &Vec<BasicInstruction>, mem_size: usize) {
    let (simple_loops, non_simple_loops) = {
        let mut simple_loops: Vec<(usize, usize)> = vec![];
        // (loop_start, ptr_change, data_change)
        let mut simple_loop: (Option<usize>, i32, i32) = (None, 0, 0);

        let mut non_simple_loops: Vec<(usize, usize)> = vec![];
        let mut non_simple_loop: Option<usize> = None;

        for idx in 0..src.len() {
            match src[idx].instr {
                BasicInstructionType::Left => simple_loop.1 -= 1,
                BasicInstructionType::Right => simple_loop.1 += 1,
                BasicInstructionType::Decr => simple_loop.2 -= 1,
                BasicInstructionType::Incr => simple_loop.2 += 1,
                BasicInstructionType::Read => simple_loop.0 = None,
                BasicInstructionType::Write => simple_loop.0 = None,
                BasicInstructionType::LBrace(_) => {
                    simple_loop = (Some(idx), 0, 0);
                    non_simple_loop = Some(idx)
                }
                BasicInstructionType::RBrace(_) => {
                    if let (Some(old_idx), ptr_change, data_change) = simple_loop {
                        if ptr_change == 0 && (data_change == 1 || data_change == -1) {
                            simple_loops.push((old_idx, idx));
                        } else if let Some(old_idx) = non_simple_loop {
                            non_simple_loops.push((old_idx, idx));
                        }
                    } else if let Some(old_idx) = non_simple_loop {
                        non_simple_loops.push((old_idx, idx));
                    }
                    simple_loop.0 = None;
                    non_simple_loop = None;
                }
            }
        }

        (simple_loops, non_simple_loops)
    };

    let mut state = RuntimeState::new(mem_size);

    let mut counts: Vec<usize> = vec![0; src.len()];

    while state.instr < src.len() {
        counts[state.instr] += 1;

        match src[state.instr].instr {
            BasicInstructionType::Left => state.ptr -= 1,
            BasicInstructionType::Right => state.ptr += 1,
            BasicInstructionType::Decr => {
                state.memory[state.ptr] = state.memory[state.ptr].wrapping_sub(1)
            }
            BasicInstructionType::Incr => {
                state.memory[state.ptr] = state.memory[state.ptr].wrapping_add(1)
            }
            BasicInstructionType::Read => {
                if let Some(Ok(ch)) = std::io::stdin().bytes().next() {
                    state.memory[state.ptr] = ch;
                } else {
                    state.memory[state.ptr] = 0;
                };
            }
            BasicInstructionType::Write => print!("{}", state.memory[state.ptr] as char),
            BasicInstructionType::LBrace(instr) => {
                if state.memory[state.ptr] == 0 {
                    counts[instr] += 1;
                    state.instr = instr
                }
            }
            BasicInstructionType::RBrace(instr) => {
                if state.memory[state.ptr] != 0 {
                    counts[instr] += 1;
                    state.instr = instr
                }
            }
        }
        state.instr += 1;
    }

    println!("Instruction counts:");

    for i in 0..src.len() {
        println!(
            "    {} : {} : {}",
            src[i].position, src[i].ch as char, counts[i]
        );
    }

    println!();
    println!("Simple loops:");

    let simple_loops = {
        let mut simple_loops = simple_loops.iter().collect::<Vec<&(usize, usize)>>();
        simple_loops.sort_by(|l1, l2| (&counts[l2.0]).cmp(&counts[l1.0]));
        simple_loops
    };
    for simple_loop in simple_loops {
        println!(
            r"    {}: `{}`: {}",
            simple_loop.0,
            src[simple_loop.0..=simple_loop.1]
                .iter()
                .map(|instr| (instr.ch as char).to_string())
                .collect::<Vec<String>>()
                .join(""),
            counts[simple_loop.0]
        );
    }

    println!();
    println!("Non-simple innermost loops:");

    let non_simple_loops = {
        let mut non_simple_loops = non_simple_loops.iter().collect::<Vec<&(usize, usize)>>();
        non_simple_loops.sort_by(|l1, l2| (&counts[l2.0]).cmp(&counts[l1.0]));
        non_simple_loops
    };
    for non_simple_loop in non_simple_loops {
        println!(
            r"    {}: `{}`: {}",
            non_simple_loop.0,
            src[non_simple_loop.0..=non_simple_loop.1]
                .iter()
                .map(|instr| (instr.ch as char).to_string())
                .collect::<Vec<String>>()
                .join(""),
            counts[non_simple_loop.0]
        );
    }
}
