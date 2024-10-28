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

    /// The size of the function "prologue", which retrieves the memory tape pointer
    prologue_size: usize,

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
            Mov(Register(R12), Register(RDI)),
        ])?;
        let prologue_size = fn_prologue.len();
        fn_mem.extend(fn_prologue.into_iter());

        let fn_ptr = unsafe { mem::transmute(fn_mem.contents) };
        let instr_blocks = AMD64Instruction::convert_instructions(instrs)
            .into_iter()
            .rev()
            .collect();

        Ok(Self {
            fn_mem,
            fn_ptr,
            prologue_size,
            instr_blocks,
        })
    }

    /// Runs this JIT-compiled program
    pub fn run(&mut self, memory_center: *mut libc::c_void) -> BFResult<()> {
        // A mutable pointer to the current location in the memory tape.
        // This starts at the center of the memory region.
        let mut memory_ptr = memory_center;

        // This will be added to the end of the function, every time a new block is encoded.
        // "Return the current location in the memory tape"
        let fn_epilogue =
            AMD64Instruction::encode_block(&vec![Mov(Register(RAX), Register(R12)), Ret()])?;

        let jmp_size = Jmp(0, None).to_binary()?.len();
        self.fn_mem.size += jmp_size + fn_epilogue.len();

        // For each block of instructions in the program:
        while let Some(block) = self.instr_blocks.pop() {
            self.fn_mem.size -= fn_epilogue.len();

            // Encode an unconditional jump to the start of the new block
            let jump = Jmp(
                (self.fn_mem.size - self.prologue_size - jmp_size) as isize,
                None,
            )
            .to_binary()?;
            for byte_index in 0..jump.len() {
                self.fn_mem[self.prologue_size + byte_index] = jump[byte_index];
            }

            // JIT-compile the instructions in the new block (plus the added epilogue)
            let bytes = AMD64Instruction::encode_block(&*block)?
                .into_iter()
                .chain(fn_epilogue.clone());

            // Save the encoded instructions to the executable memory
            self.fn_mem.extend(bytes);

            // Finally, we can just call this as an FFI function
            memory_ptr = (self.fn_ptr)(memory_ptr);
        }

        Ok(())
    }
}
