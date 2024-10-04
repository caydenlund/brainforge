//! Types to represent instructions in the BF intermediate representation

use crate::{BFError, BFParseError, BFResult};

/// A single instruction in the BF intermediate representation
#[derive(Clone, Debug)]
pub enum IntermediateInstruction {
    /// A loop of instructions
    Loop(Vec<IntermediateInstruction>),

    /// Adds the multiplied value of the source cell to the target cell
    ///
    /// `(target, multiplier)`
    AddDynamic(i32, i32),

    /// Sets the given cell to 0
    Zero,

    /// A simple loop of instructions
    SimpleLoop(Vec<IntermediateInstruction>),

    /// Moves the data pointer by the given offset
    Move(i32),

    /// Adds the given offset to the cell at the current data pointer
    Add(i32),

    /// Reads a value from stdin into the current cell
    Read,

    /// Writes the value at the current cell to stdout
    Write,

    /// Performs a memory scan of the given stride
    Scan(i32),
}

impl IntermediateInstruction {
    /// Given an array of bytes, parse it into a vector of instructions
    pub fn parse_instrs(src: &[u8]) -> BFResult<Vec<IntermediateInstruction>> {
        let mut instrs: Vec<Vec<IntermediateInstruction>> = vec![];
        let mut cur_instrs: Vec<IntermediateInstruction> = vec![];

        for position in 0..src.len() {
            match src[position] {
                b'<' => {
                    cur_instrs.push(IntermediateInstruction::Move(-1));
                }
                b'>' => {
                    cur_instrs.push(IntermediateInstruction::Move(1));
                }
                b'-' => {
                    cur_instrs.push(IntermediateInstruction::Add(-1));
                }
                b'+' => {
                    cur_instrs.push(IntermediateInstruction::Add(1));
                }
                b',' => {
                    cur_instrs.push(IntermediateInstruction::Read);
                }
                b'.' => {
                    cur_instrs.push(IntermediateInstruction::Write);
                }
                b'[' => {
                    instrs.push(cur_instrs);
                    cur_instrs = vec![];
                }
                b']' => {
                    let new_instr = IntermediateInstruction::Loop(cur_instrs);
                    if let Some(old_instrs) = instrs.pop() {
                        cur_instrs = old_instrs;
                        cur_instrs.push(new_instr);
                    } else {
                        return Err(BFError::ParseError(BFParseError::UnmatchedRBrace(position)));
                    }
                }
                _ => {}
            };
        }

        if instrs.len() == 0 {
            Ok(cur_instrs)
        } else {
            Err(BFError::ParseError(BFParseError::UnmatchedLBrace(0)))
        }
    }
}
