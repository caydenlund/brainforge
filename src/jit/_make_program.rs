use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register};
use crate::assembly::Instruction;
use crate::generator::Architecture;
use crate::instruction::IntermediateInstruction;
use crate::jit::{JitMem, PAGE_SIZE};

use std::mem;

pub fn make_program(
    instrs: &[IntermediateInstruction],
    arch: &Architecture,
) -> fn(*mut libc::c_void) {
    let bytes = match arch {
        Architecture::AMD64 => {
            use AMD64Instruction::*;
            use AMD64Operand::*;
            use AMD64Register::*;

            let mut result = vec![];
            result.extend(
                vec![
                    Mov(Register(R12), Register(RDI)),
                    Add(Register(ECX), Memory(None, Some(RCX), Some(RDX), None, None)),
                    Add(Register(ECX), Memory(None, Some(RCX), Some(RDX), Some(2), None)),
                    Add(Register(ECX), Memory(None, Some(RCX), Some(RDX), Some(4), None)),
                    // TODO: YMM reg initialization
                ]
                .iter()
                .map(|instr| instr.to_binary()),
            );
            result.extend(
                instrs
                    .iter()
                    .map(|instr| AMD64Instruction::bf_to_binary(instr)),
            );
            result.push(vec![0xC3]);
            result.concat()
        }
    };

    let mut mem = JitMem::new((bytes.len() - 1) / PAGE_SIZE + 1);

    mem.extend(bytes.into_iter());

    unsafe { mem::transmute(mem.contents) }
}
