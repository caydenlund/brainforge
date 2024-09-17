//! Defines the data structure to represent the state of the runtime at any given moment
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

/// The current state of the interpreter at runtime
pub struct RuntimeState {
    /// The instruction pointer, as an index into an array of instructions
    pub instr: usize,

    /// The array of memory
    pub memory: Vec<u8>,

    /// The memory pointer, as an index into `self.memory`
    pub ptr: usize,
}

impl RuntimeState {
    /// Initializes a new [`RuntimeState`] object
    pub fn new(mem_size: usize) -> Self {
        Self {
            instr: 0,
            memory: vec![0; mem_size],
            ptr: 0,
        }
    }
}
