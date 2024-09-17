//! Types to represent a single BF instruction, plus parsing methods
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use crate::{BFError, BFParseError, BFResult};

/// The semantics of a single BF instruction
///
/// This is used instead of working with the source characters directly, because I intend to add
/// optimizations
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instr {
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

/// Represents a single BF instruction in the source program
///
/// Tracks not only the semantics of the instruction, but also the original char. and index
#[derive(Clone, Debug)]
pub struct Instruction {
    /// The [`Instr`] of this BF instruction
    pub instr: Instr,

    /// The original character from the source input
    pub ch: u8,

    /// The position (character index) of this instruction in the source input
    pub position: usize,
}

impl Instruction {
    /// Given a slice of bytes, parses it into a vector of [`Instruction`]s
    ///
    /// Each byte in the source input is read and individually handled.
    /// This method will panic if the source input contains unmatched
    /// [`Instruction::LBrace`] or [`Instruction::RBrace`] instructions
    pub fn parse_instrs(src: &[u8]) -> BFResult<Vec<Instruction>> {
        let mut instrs: Vec<Instruction> = vec![];
        let mut open: Vec<usize> = vec![];

        for position in 0..src.len() {
            let ch = src[position];
            let instr = match ch {
                b'<' => Some(Instr::Left),
                b'>' => Some(Instr::Right),
                b'-' => {
                    Some(Instr::Decr)
                }
                b'+' => {
                    Some(Instr::Incr)
                }
                b',' => {
                    Some(Instr::Read)
                }
                b'.' => {
                    Some(Instr::Write)
                }
                b'[' => {
                    open.push(instrs.len());
                    Some(Instr::LBrace(0))
                }
                b']' => {
                    let Some(old_open) = open.pop() else {
                        return Err(BFError::ParseError(BFParseError::UnmatchedRBrace(position)));
                    };

                    instrs[old_open].instr = Instr::LBrace(instrs.len());
                    Some(Instr::RBrace(old_open))
                }
                _ => None,
            };

            if let Some(instr) = instr {
                instrs.push(Instruction {
                    instr,
                    position,
                    ch,
                });
            }
        }

        if let Some(idx) = open.pop() {
            return Err(BFError::ParseError(BFParseError::UnmatchedLBrace(idx)));
        }

        Ok(instrs)
    }
}
