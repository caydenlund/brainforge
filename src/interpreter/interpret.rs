//! Defines the procedures to run a BF program
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use super::RuntimeState;
use crate::instruction::Instruction;

/// Interprets the given BF instructions
pub fn interpret(src: &Vec<Instruction>, mem_size: usize) {
    let mut state = RuntimeState::new(mem_size);

    while state.instr < src.len() {
        match src[state.instr] {
            Instruction::Left => state.ptr -= 1,
            Instruction::Right => state.ptr += 1,
            Instruction::Decr => state.memory[state.ptr] = state.memory[state.ptr].wrapping_sub(1),
            Instruction::Incr => state.memory[state.ptr] = state.memory[state.ptr].wrapping_add(1),
            Instruction::Read => todo!(),
            Instruction::Write => print!("{}", state.memory[state.ptr] as char),
            Instruction::LBrace(instr) => {
                if state.memory[state.ptr] == 0 {
                    state.instr = instr
                }
            }
            Instruction::RBrace(instr) => {
                if state.memory[state.ptr] != 0 {
                    state.instr = instr
                }
            }
        }
        state.instr += 1;
    }
}
