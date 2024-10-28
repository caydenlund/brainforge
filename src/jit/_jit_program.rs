//! Driver for compiling and running a BF program just-in-time

use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register};
use crate::instruction::IntermediateInstruction;
use crate::jit::JitMem;
use crate::BFResult;
use AMD64Instruction::*;
use AMD64Operand::*;
use AMD64Register::*;

use std::mem;

/// A JIT-compiled program
pub struct JitProgram {
    /// A region of executable memory
    fn_mem: JitMem,

    /// A callable function pointer
    ///
    /// The argument is a pointer to a pointer to the current location in the memory tape.
    /// This makes the memory tape location preservable across function calls.
    fn_ptr: fn(*mut libc::c_void) -> *mut libc::c_void,

    /// Blocks of non-encoded assembly instructions
    instr_blocks: Vec<Vec<AMD64Instruction>>,
}

impl JitProgram {
    /// Creates a new JIT-compiled program for the given instructions
    pub fn new(instrs: &[IntermediateInstruction], num_pages: usize) -> BFResult<Self> {
        // Should be plenty of room
        let mut fn_mem = JitMem::new(num_pages);

        let fn_prologue = AMD64Instruction::encode_block(&vec![
            // Copy the given memory tape location (fn argument) into register R12
            Push(Register(R12)),
            Push(Register(R13)),
            Mov(Register(R12), Register(RDI)),
        ])?;
        fn_mem.extend(fn_prologue.into_iter());

        let fn_ptr = unsafe { mem::transmute(fn_mem.contents) };
        let instr_blocks = AMD64Instruction::convert_instructions(instrs)
            .into_iter()
            .rev()
            .collect::<Vec<_>>();

        Ok(Self {
            fn_mem,
            fn_ptr,
            instr_blocks,
        })
    }

    /// Runs this JIT-compiled program
    pub fn run(&mut self, memory_center: *mut libc::c_void) -> BFResult<()> {
        // A mutable pointer to the current location in the memory tape.
        // This starts at the center of the memory tape.
        let mut memory_ptr = memory_center;

        // This will be added to the end of the function every time a new block is encoded.
        // It returns the current memory tape pointer in register RAX and restores saved registers
        // before returning.
        let fn_epilogue = AMD64Instruction::encode_block(&vec![
            Mov(Register(RAX), Register(R12)),
            Pop(Register(R13)),
            Pop(Register(R12)),
            Ret(),
        ])?;

        // We add an unconditional `jmp` instruction in the start of the function,
        // which jumps directly to the start of the new block.
        let pre_jump_position = self.fn_mem.position;
        let jmp_size = Jmp(0, None).to_binary()?.len();
        let post_block_position = pre_jump_position + jmp_size;

        let mut next_block_position = post_block_position;

        // For each block of instructions in the program:
        while let Some(block) = self.instr_blocks.pop() {
            // Encode the unconditional jump to the start of the new block
            let jump =
                Jmp((next_block_position - post_block_position) as isize, None).to_binary()?;
            for byte_index in 0..jump.len() {
                self.fn_mem[pre_jump_position + byte_index] = jump[byte_index];
            }

            // JIT-compile the instructions in the new block
            let bytes = AMD64Instruction::encode_block(&*block)?;

            // Save the encoded instructions to the executable memory
            self.fn_mem.position = next_block_position;
            self.fn_mem.extend(bytes.into_iter());
            next_block_position = self.fn_mem.position;
            // Also, add the epilogue that restores saved registers and returns the new mem. pointer
            self.fn_mem.extend(fn_epilogue.clone().into_iter());

            // Finally, we can just call this as an FFI function
            memory_ptr = (self.fn_ptr)(memory_ptr);
        }

        Ok(())
    }
}
