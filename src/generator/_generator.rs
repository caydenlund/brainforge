//! Defines the [`Generator`] trait

use crate::instruction::BasicInstruction;

/// An assembly-code generator from BF programs
pub trait Generator {
    /// Constructs a new [`Generator`] for the given program and memory size
    fn new(src: &[BasicInstruction], mem_size: usize) -> Self;

    /// Reports the comprehensive assembly text for this generator's program
    fn text(&self) -> String;
}
