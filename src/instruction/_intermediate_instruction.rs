//! Types to represent instructions in the BF intermediate representation

use crate::{BFError, BFParseError, BFResult};

/// Options governing parsing instructions
pub struct ParseOpts {
    /// Whether to coalesce adjacent matching instructions
    pub coalesce: bool,
}

impl ParseOpts {
    /// Constructs a new `ParseOpts` instance
    pub fn new() -> Self {
        Self { coalesce: false }
    }

    /// Sets the `coalesce` field to the given value
    pub fn coalesce(mut self, value: bool) -> Self {
        self.coalesce = value;
        self
    }
}

/// A single instruction in the BF intermediate representation
pub enum IntermediateInstruction {
    /// A loop of instructions
    Loop(Vec<IntermediateInstruction>),

    // BasicLoop(Vec<(i32, i32)>),

    /// Moves the data pointer by the given offset
    Move(i32),

    /// Adds the given offset to the cell at the current data pointer
    Add(i32),

    /// Reads a value from stdin into the current cell
    Read,

    /// Writes the value at the current cell to stdout
    Write,
}

impl IntermediateInstruction {
    /// Given an array of bytes, parse it into a vector of instructions
    pub fn parse_instrs(src: &[u8], opts: ParseOpts) -> BFResult<Vec<IntermediateInstruction>> {
        let mut instrs: Vec<Vec<IntermediateInstruction>> = vec![];
        let mut cur_instrs: Vec<IntermediateInstruction> = vec![];

        for position in 0..src.len() {
            match src[position] {
                b'<' => {
                    if opts.coalesce && cur_instrs.len() > 0 {
                        let last_ind = cur_instrs.len() - 1;
                        if let IntermediateInstruction::Move(offset) = cur_instrs[last_ind] {
                            cur_instrs[last_ind] = IntermediateInstruction::Move(offset - 1);
                        }
                    } else {
                        cur_instrs.push(IntermediateInstruction::Move(-1));
                    }
                }
                b'>' => {
                    if opts.coalesce && cur_instrs.len() > 0 {
                        let last_ind = cur_instrs.len() - 1;
                        if let IntermediateInstruction::Move(offset) = cur_instrs[last_ind] {
                            cur_instrs[last_ind] = IntermediateInstruction::Move(offset + 1);
                        }
                    } else {
                        cur_instrs.push(IntermediateInstruction::Move(1));
                    }
                }
                b'-' => {
                    if opts.coalesce && cur_instrs.len() > 0 {
                        let last_ind = cur_instrs.len() - 1;
                        if let IntermediateInstruction::Add(offset) = cur_instrs[last_ind] {
                            cur_instrs[last_ind] = IntermediateInstruction::Add(offset - 1);
                        }
                    } else {
                        cur_instrs.push(IntermediateInstruction::Add(-1));
                    }
                }
                b'+' => {
                    if opts.coalesce && cur_instrs.len() > 0 {
                        let last_ind = cur_instrs.len() - 1;
                        if let IntermediateInstruction::Add(offset) = cur_instrs[last_ind] {
                            cur_instrs[last_ind] = IntermediateInstruction::Add(offset + 1);
                        }
                    } else {
                        cur_instrs.push(IntermediateInstruction::Add(1));
                    }
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
