//! Types to represent a single BF instruction, plus parsing methods
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use crate::{BFError, BFParseError, BFResult};

/// A single BF instruction
///
/// This is used instead of working with the source characters directly, because I intend to add
/// optimizations
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// A BF `<` instruction
    ///
    /// Moves the memory pointer one to the left
    Left,

    /// A BF `>` instruction
    ///
    /// Moves the memory pointer one to the right
    Right,

    /// A BF `-` instruction
    ///
    /// Decrements the cell at the current memory pointer
    Decr,

    /// A BF `+` instruction
    ///
    /// Increments the cell at the current memory pointer
    Incr,

    /// A BF `,` instruction
    ///
    /// Reads a single byte of input and stores it in the current cell
    Read,

    /// A BF `.` instruction
    ///
    /// Writes the value in the current cell as a char to stdout
    Write,

    /// A BF `[` instruction
    ///
    /// If the value in the current cell is 0, then jump to the matching `]` instruction
    /// This tuple-like variant holds one field: the index of the matching `]` instruction
    LBrace(usize),

    /// A BF `]` instruction
    ///
    /// If the value in the current cell is not 0, then jump to the matching `[` instruction
    /// This tuple-like variant holds one field: the index of the matching `[` instruction
    RBrace(usize),
}

impl Instruction {
    /// Given a slice of bytes, parses it into a vector of [`Instruction`]s
    ///
    /// Each byte in the source input is read and individually handled
    /// This method will panic if the source input contains unmatched
    /// [`Instruction::LBrace`] or [`Instruction::RBrace`] instructions
    pub fn parse_instrs(src: &[u8]) -> BFResult<Vec<Instruction>> {
        let mut instrs: Vec<Instruction> = vec![];
        let mut open: Vec<usize> = vec![];

        for idx in 0..src.len() {
            match src[idx] {
                b'<' => instrs.push(Instruction::Left),
                b'>' => instrs.push(Instruction::Right),
                b'-' => instrs.push(Instruction::Decr),
                b'+' => instrs.push(Instruction::Incr),
                b',' => instrs.push(Instruction::Read),
                b'.' => instrs.push(Instruction::Write),
                b'[' => {
                    open.push(instrs.len());
                    instrs.push(Instruction::LBrace(0));
                }
                b']' => {
                    let Some(old_open) = open.pop() else {
                        return Err(BFError::ParseError(BFParseError::UnmatchedRBrace(idx)));
                    };

                    instrs[old_open] = Instruction::LBrace(instrs.len());
                    instrs.push(Instruction::RBrace(old_open));
                }
                _ => {}
            };
        }

        if let Some(idx) = open.pop() {
            return Err(BFError::ParseError(BFParseError::UnmatchedLBrace(idx)));
        }

        Ok(instrs)
    }
}
