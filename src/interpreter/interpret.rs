//! Defines the procedures to run a BF program
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use super::RuntimeState;
use crate::instruction::{Instr, Instruction};

/// Interprets the given BF instructions
pub fn interpret(src: &Vec<Instruction>, mem_size: usize) {
    let mut state = RuntimeState::new(mem_size);

    while state.instr < src.len() {
        match src[state.instr].instr {
            Instr::Left => state.ptr -= 1,
            Instr::Right => state.ptr += 1,
            Instr::Decr => state.memory[state.ptr] = state.memory[state.ptr].wrapping_sub(1),
            Instr::Incr => state.memory[state.ptr] = state.memory[state.ptr].wrapping_add(1),
            Instr::Read => todo!(),
            Instr::Write => print!("{}", state.memory[state.ptr] as char),
            Instr::LBrace(instr) => {
                if state.memory[state.ptr] == 0 {
                    state.instr = instr
                }
            }
            Instr::RBrace(instr) => {
                if state.memory[state.ptr] != 0 {
                    state.instr = instr
                }
            }
        }
        state.instr += 1;
    }
}

/// Interprets the given BF instructions, with added profiling
pub fn interpret_profile(
    src: &Vec<Instruction>,
    mem_size: usize,
    simple_loops: Vec<(usize, usize)>,
) {
    let mut state = RuntimeState::new(mem_size);

    let mut counts: Vec<usize> = vec![0; src.len()];

    while state.instr < src.len() {
        counts[state.instr] += 1;

        match src[state.instr].instr {
            Instr::Left => state.ptr -= 1,
            Instr::Right => state.ptr += 1,
            Instr::Decr => state.memory[state.ptr] = state.memory[state.ptr].wrapping_sub(1),
            Instr::Incr => state.memory[state.ptr] = state.memory[state.ptr].wrapping_add(1),
            Instr::Read => todo!(),
            Instr::Write => print!("{}", state.memory[state.ptr] as char),
            Instr::LBrace(instr) => {
                if state.memory[state.ptr] == 0 {
                    counts[instr] += 1;
                    state.instr = instr
                }
            }
            Instr::RBrace(instr) => {
                if state.memory[state.ptr] != 0 {
                    counts[instr] += 1;
                    state.instr = instr
                }
            }
        }
        state.instr += 1;
    }

    for i in 0..src.len() {
        println!(
            "{} : {} : {}",
            src[i].position, src[i].ch as char, counts[i]
        );
    }

    for simple_loop in simple_loops.iter().map(|s_loop| counts[s_loop.0]) {
        println!("{}", simple_loop);
    }
}
