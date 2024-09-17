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
    non_simple_loops: Vec<(usize, usize)>,
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
