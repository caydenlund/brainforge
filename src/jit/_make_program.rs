use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register};
use crate::generator::Architecture;
use crate::instruction::IntermediateInstruction;
use crate::jit::{JitMem, PAGE_SIZE};

use crate::BFResult;
use std::mem;

pub fn make_program(
    instrs: &[IntermediateInstruction],
    arch: &Architecture,
) -> BFResult<fn(*mut libc::c_void)> {
    let bytes: Vec<u8> = match arch {
        Architecture::AMD64 => {
            use AMD64Instruction::*;
            use AMD64Operand::*;
            use AMD64Register::*;

            vec![
                Mov(Register(R12), Register(RDI)),
                // TODO: YMM reg initialization
            ]
            .iter()
            .map(|instr| instr.to_binary())
            .chain(
                instrs
                    .iter()
                    .map(|instr| AMD64Instruction::bf_to_binary(instr)),
            )
            .chain(vec![Ok(vec![0xC3])])
            .collect::<BFResult<Vec<Vec<u8>>>>()
            .map(|list| list.concat())
        }
    }?;

    let mut mem = JitMem::new((bytes.len() - 1) / PAGE_SIZE + 1);

    mem.extend(bytes.into_iter());

    Ok(unsafe { mem::transmute(mem.contents) })
}
