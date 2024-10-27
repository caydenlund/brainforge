use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register};
use crate::instruction::IntermediateInstruction;
use crate::jit::JitMem;
use crate::BFResult;
use AMD64Instruction::*;
use AMD64Operand::*;
use AMD64Register::*;

use std::mem;

/// A JIT-compiled program
pub struct Program {
    /// A region of executable memory
    fn_mem: JitMem,

    /// A callable function pointer
    ///
    /// The argument is a pointer to a pointer to the current location in the memory tape.
    /// This makes the memory tape location preservable across function calls.
    fn_ptr: fn(*mut *mut libc::c_void),

    /// The size of the function "prologue", which retrieves the memory tape pointer
    prologue_size: usize,

    /// Blocks of non-encoded assembly instructions
    instr_blocks: Vec<Vec<AMD64Instruction>>,
}

impl Program {
    /// Creates a new JIT-compiled program for the given instructions
    pub fn new(instrs: &[IntermediateInstruction]) -> BFResult<Self> {
        let mut fn_mem = JitMem::new(10);

        let fn_prolog = AMD64Instruction::encode_block(&vec![
            Mov(Register(R12), Memory(None, Some(RDI), None, None, None)),
            Mov(Register(R14), Register(RDI)),
        ])?;
        let prolog_size = fn_prolog.len();
        fn_mem.extend(fn_prolog.into_iter());

        let fn_ptr = unsafe { mem::transmute(fn_mem.contents) };
        let instr_blocks = AMD64Instruction::convert_instructions(instrs)
            .into_iter()
            .rev()
            .collect();

        Ok(Self {
            fn_mem,
            fn_ptr,
            prologue_size: prolog_size,
            instr_blocks,
        })
    }

    /// Runs this JIT-compiled program
    pub fn run(&mut self, memory_center: *mut libc::c_void) -> BFResult<()> {
        // Local mutable copy
        let mut memory_ptr = memory_center;

        // Save the current memory pointer to the `memory_ptr` variable above before returning
        let fn_epilogue = AMD64Instruction::encode_block(&vec![
            Mov(Memory(None, Some(R14), None, None, None), Register(R12)),
            Ret(),
        ])?;

        // For each block of instructions in the program...
        while let Some(block) = self.instr_blocks.pop() {
            // ... JIT-compile the instructions...
            let bytes = AMD64Instruction::encode_block(&*block)?
                .into_iter()
                .chain(fn_epilogue.clone().into_iter());
            // ... and save the encoded instructions to the executable memory
            self.fn_mem.size = self.prologue_size;
            self.fn_mem.extend(bytes);

            // Finally, we can just call this as an FFI function
            (self.fn_ptr)(&mut memory_ptr);
        }

        Ok(())
    }
}
